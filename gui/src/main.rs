use orbtk::prelude::*;

fn main() {
    Application::new()
        .window(|ctx| {
            Window::create()
                .title("helloworld")
                .position((100.0, 100.0))
                .size(420.0, 400.0)
                .child(TextBlock::create().text("Hello").build(ctx))
                .build(ctx)
        })
        .run();
}