/*
 * This example builds an orbtk GUI (https://github.com/redox-os/orbtk/) with 2 buttons:
 * Button 1 (S): Runs some syncronous code (defined in do_sync below and contains a sleep) and shows how the GUI hangs
 * Button 2 (A): Runs the same sleep code in a separate thread (defined in do_async and main) and shows how the GUI doesn't hang
 *               Clicking this button shows the async code display to stdout in the terminal from
 *               which the GUI was built/launched. Clicking it multiple times will repeat the async
 *               code.
 *
 * This was based on the calculator example in orbtk repo
 */
use orbtk::{
    prelude::*
};
use futures::executor::ThreadPool;
use crossbeam::crossbeam_channel::{Sender};

#[derive(Debug, Clone)]
pub struct MySender(Sender<u32>);
type FutureSender = Option<MySender>;

impl PartialEq for MySender {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

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
    fn action(&mut self, x: Action) {
        self.action = Some(x);
    }

    fn do_sync(&mut self, ctx: &mut Context) {
        main_view(ctx.widget()).set_text("Sync: Sleep 3 seconds"); // This never really shows up because the GUI loop never moves on to display it before the wake up display below
        let sleep_dur1 = std::time::Duration::from_secs(3);
        std::thread::sleep(sleep_dur1);
        main_view(ctx.widget()).set_text("Sync: wake up");
    }

    fn do_async(&mut self, ctx: &mut Context) {
        //\n(also check terminal stdout for async thread)
        main_view(ctx.widget()).set_text("Async main: Sleep 3 seconds");
        self.future_sender.as_ref().unwrap().0.send(1);
        main_view(ctx.widget()).set_text("Async main: wake up");
    }
}

impl State for MainViewState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if self.future_sender.is_none() {
            self.future_sender = ctx.widget().get::<FutureSender>("future_sender").clone();
        }

        if let Some(action) = self.action {
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
    text: String16,
    future_sender: FutureSender
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
                                    .child(generate_operation_button(ctx, id, 'S', 0, 5, 0, Action::Sync))
                                    .child(generate_operation_button(ctx, id, 'A', 6, 3, 0, Action::Async))
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
        while let v2 = r2.recv() {
        match v2 {
          Ok(_f) => {
            println!("Async thread: Sleep 3 seconds");
            let sleep_dur1 = std::time::Duration::from_secs(3);
            std::thread::sleep(sleep_dur1);
            println!("Async thread: wake up");
          },
          Err(e) => {
            println!("tw custom worker thread error: {}", e);
          }
        }
        }
    };

    let pool = ThreadPool::new().unwrap();
    pool.spawn_ok(future);

    let future_sender: FutureSender = Some(MySender(tx));

    Application::new()
        .window(move |ctx| {
            let mut main_view = MainView::new();
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
