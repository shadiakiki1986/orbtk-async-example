// Based on the calculator example in orbtk repo
use orbtk::{
    prelude::*
};

#[derive(Default, AsAny)]
pub struct MainViewState {
    input: String,
    left_side: Option<f64>,
    action: Option<int>,
}

impl MainViewState {
    fn action(&mut self) {
        self.action = Some(1);
    }

    fn calculate(&mut self, ctx: &mut Context) {
        let mut result = 0.0;
            if let Some(left_side) = self.left_side {
                result = left_side + 1;
            }
        }

        main_view(ctx.widget()).set_text(format!("{}", result));

        self.left_side = Some(result);
    }
}

impl State for MainViewState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.action {
            self.calculate();
            self.action = None;
        }
    }
}

fn generate_operation_button(
    ctx: &mut BuildContext,
    id: Entity,
    sight: char,
    primary: bool,
    column: usize,
    column_span: usize,
    row: usize,
) -> Entity {
    let button = Button::new()
        .min_size(48.0, 48.0)
        .text(sight.to_string())
        .on_click(move |states, _| -> bool {
            state(id, states).action();
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
                                    .child(generate_operation_button(ctx, id, 'C', false, 0, 5, 0))
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}

fn main() {
    Application::new()
        .window(|ctx| {
            Window::new()
                .title("OrbTk - Calculator example")
                .position((100.0, 100.0))
                .size(212.0, 336.0)
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}

// helper to request MainViewState
fn state<'a>(id: Entity, states: &'a mut StatesContext) -> &'a mut MainViewState {
    states.get_mut(id)
}
