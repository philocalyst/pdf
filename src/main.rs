use xilem::masonry::properties::types::Length;
use xilem::style::{Padding, Style};
use xilem::view::{
    VirtualScroll, canvas, flex_col, sized_box, unlimited_virtual_scroll, virtual_scroll,
};
use xilem::winit::error::EventLoopError;
use xilem::{Affine, EventLoop, EventLoopBuilder, WidgetView, WindowOptions, Xilem};

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

fn render_svg_to_canvas(svg_str: &'static str) -> impl WidgetView<MainState> {
    // Parse SVG and generate Vello scene
    let svg_scene = vello_svg::render(svg_str).unwrap();
    let tree = usvg::Tree::from_str(svg_str, &usvg::Options::default()).unwrap();
    let svg_size = tree.size();

    sized_box(canvas(move |scene, size| {
        // Compute scaling
        let scale_x = size.width / svg_size.width() as f64;

        // Uniform scaling about SVG center
        let transform =
            Affine::scale_about(scale_x, (svg_size.width() / 2.0, svg_size.height() / 2.0));

        scene.append(&svg_scene, Some(transform));
    }))
    .height(Length::px(svg_size.height() as f64))
}

fn app(state: &mut MainState) -> impl WidgetView<MainState> + use<> {
    let svg1 = include_str!("/Users/philocalyst/Downloads/hayro/hayro-svg/rendered_0.svg");
    let svg2 = include_str!("/Users/philocalyst/Downloads/hayro/hayro-svg/examples/rendered_0.svg");

    virtual_scroll(0..2, move |state, index| {
        let svg = match index {
            0 => svg1,
            1 => svg2,
            _ => unreachable!(),
        };
        render_svg_to_canvas(svg)
    })
}
