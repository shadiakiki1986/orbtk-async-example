// Based on the calculator example in orbtk repo
use orbtk::{
    prelude::*
};
use futures::executor::ThreadPool;
use crossbeam::crossbeam_channel::{Sender};

type FutureSender = Option<Sender<impl Future>>;

#[derive(Debug, Copy, Clone)]
enum Action {
    Sync,
    Async,
}

#[derive(Default, AsAny)]
pub struct MainViewState {
    action: Option<Action>,
    future_sender: FutureSender,
}

impl MainViewState {
    fn action(&mut self, Action x) {
        self.action = Some(x);
    }

    fn do_sync(&mut self, ctx: &mut Context) {
        main_view(ctx.widget()).set_text(format!("Sync: Sleep 3 seconds"));
        let sleep_dur1 = std::time::Duration::from_secs(3);
        std::thread::sleep(sleep_dur1);
        main_view(ctx.widget()).set_text(format!("Sync: wake up"));
    }

    fn do_async(&mut self, ctx: &mut Context) {
        let future = async {
            println!("Async thread: Sleep 3 seconds");
            let sleep_dur1 = std::time::Duration::from_secs(3);
            std::thread::sleep(sleep_dur1);
            println!("Async thread: wake up");
        };

        main_view(ctx.widget()).set_text(format!("Async main: Sleep 3 seconds (also check terminal stdout for async thread)"));
        self.future_sender.send(future);
        main_view(ctx.widget()).set_text(format!("Async main: wake up (also check terminal stdout for async thread)"));
    }
}

impl State for MainViewState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(_action) = self.action {
            match action {
                Action::Sync => {
                    self.do_sync(ctx);
                },
                _ => {
                    self.do_async(ctx);
                }
            }
            self.action = None;
        }
    }
}

fn generate_operation_button(
    ctx: &mut BuildContext,
    id: Entity,
    sight: char,
    column: usize,
    column_span: usize,
    row: usize,
    action_type: Action
) -> Entity {
    let button = Button::new()
        .min_size(48.0, 48.0)
        .text(sight.to_string())
        .on_click(move |states, _| -> bool {
            state(id, states).action(action_type);
            true
        })
        .attach(Grid::column(column))
        .attach(Grid::column_span(column_span))
        .attach(Grid::row(row));
    button.build(ctx)
}

widget!(MainView<MainViewState> {
    text: String16
});

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView")
            .width(212.0)
            .height(336.0)
            .text("")
            .child(
                Grid::new()
                    .rows(Rows::new().add(72.0).add("*"))
                    .child(
                        Container::new()
                            .padding(8.0)
                            .style("header_area")
                            .attach(Grid::row(0))
                            .child(
                                Grid::new()
                                    .child(
                                        ScrollViewer::new()
                                            .mode(("custom", "disabled"))
                                            .child(
                                                TextBlock::new()
                                                    .width(0.0)
                                                    .height(14.0)
                                                    .text("")
                                                    .style("input")
                                                    .id("input")
                                                    .v_align("start")
                                                    .build(ctx),
                                            )
                                            .build(ctx),
                                    )
                                    .child(
                                        TextBlock::new()
                                            .style("result")
                                            .text(id)
                                            .v_align("end")
                                            .h_align("end")
                                            .build(ctx),
                                    )
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .child(
                        Container::new()
                            .style("content_area")
                            .padding(4.0)
                            .attach(Grid::row(1))
                            .child(
                                Grid::new()
                                    .columns(
                                        Columns::new()
                                            .add(48.0)
                                            .add(4.0)
                                            .add(48.0)
                                            .add(4.0)
                                            .add(48.0)
                                            .add(4.0)
                                            .add(48.0),
                                    )
                                    .rows(
                                        Rows::new()
                                            .add(48.0)
                                            .add(4.0)
                                            .add(48.0)
                                            .add(4.0)
                                            .add(48.0)
                                            .add(4.0)
                                            .add(48.0)
                                            .add(4.0)
                                            .add(48.0),
                                    )
                                    // row 0
                                    .child(generate_operation_button(ctx, id, 'Sync', 0, 5, 0, Action::Sync))
                                    .child(generate_operation_button(ctx, id, 'Async', 6, 3, 0, Action::Async))
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}

fn main() {
    // before wrapping the runtime, spawn a thread for the twitter custom feed future to run
    let (tx, rx) = crossbeam::channel::bounded(0);

    // Spawn a future, on a separate thread, that listens for futures to execute
    let r2 = rx.clone();
    let future = async move {
        println!("\t\t tw custom worker thread: started");
        match r2.recv() {
          Ok(f) => {
            println!("tw custom worker thread: received future");
            f.await;
          },
          Err(e) => {
            println!("tw custom worker thread error: {}", e);
          }
        }
    }

    let pool = ThreadPool::new().unwrap();
    pool.spawn_ok(future);

    let future_sender: FutureSender = Some(tx);

    Application::new()
        .window(|ctx| {
            let main_view = MainView::new();
            main_view.future_sender = Some(PropertySource::Value(future_sender.clone()));
            Window::new()
                .title("OrbTk - async task example")
                .position((100.0, 100.0))
                .size(212.0, 336.0)
                .child(main_view.build(ctx))
                .build(ctx)
        })
        .run();
}

// helper to request MainViewState
fn state<'a>(id: Entity, states: &'a mut StatesContext) -> &'a mut MainViewState {
    states.get_mut(id)
}
