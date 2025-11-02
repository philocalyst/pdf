use xilem::winit::error::EventLoopError;
use xilem::{EventLoop, EventLoopBuilder, WidgetView, WindowOptions, Xilem};

enum MainState {
    Online,
}

fn main() -> Result<(), EventLoopError> {
    println!("Hello, world!");

    run(EventLoop::with_user_event());
    Ok(())
}

fn run(event_loop: EventLoopBuilder) -> Result<(), EventLoopError> {
    Xilem::new_simple(
        MainState::Online,
        app,
        WindowOptions::new("A PDF Application"),
    )
    .run_in(event_loop);

    Ok(())
}

fn app(state: &mut MainState) -> impl WidgetView<MainState> + use<> {
    let scene = vello_svg::render(r###""###).unwrap();
    let canvas = canvas(move |scened, size| {
        *scened = scene.clone();
    });

    canvas
}
