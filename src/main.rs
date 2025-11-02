use xilem::masonry::properties::types::Length;
use xilem::style::{Padding, Style};
use xilem::view::{
    VirtualScroll, canvas, flex_col, sized_box, unlimited_virtual_scroll, virtual_scroll,
};
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
    .run_in(event_loop)
}

fn app(state: &mut MainState) -> impl WidgetView<MainState> + use<> {
    let svg1 = include_str!("/Users/philocalyst/Downloads/hayro/hayro-svg/rendered_0.svg");
    let svg2 = include_str!("/Users/philocalyst/Downloads/hayro/hayro-svg/examples/rendered_0.svg");

    virtual_scroll(0..1, move |state, index| {
        flex_col((
            sized_box(canvas(move |scened, _| {
                let scene = vello_svg::render(svg2).unwrap();
                *scened = scene;
            }))
            .height(Length::px(300.0)),
            sized_box(canvas(move |scened, _| {
                let scene = vello_svg::render(svg1).unwrap();
                *scened = scene;
            }))
            .height(Length::px(300.0)),
        ))
        .padding(Padding::from(200.0))
    })
}
