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

fn render_svg_to_canvas(
    svg_str: &'static str,
    target_height: f64,
) -> impl WidgetView<MainState> {
    sized_box(canvas(move |scene, size| {
        // Parse SVG and generate Vello scene
        let svg_scene = vello_svg::render(svg_str).unwrap();
        let tree = usvg::Tree::from_str(svg_str, &usvg::Options::default()).unwrap();
        let svg_size = tree.size();

        // Compute scaling
        let scale_x = size.width / svg_size.width() as f64;

        let transform = 
            // Uniform scaling about SVG center
            Affine::scale_about(scale_x, (svg_size.width() / 2.0, svg_size.height() / 2.0));

        scene.append(&svg_scene, Some(transform));
    }))
    .height(Length::px(target_height))
}

fn app(state: &mut MainState) -> impl WidgetView<MainState> + use<> {
    let svg1 = include_str!("/Users/philocalyst/Downloads/hayro/hayro-svg/rendered_0.svg");
    let svg2 = include_str!("/Users/philocalyst/Downloads/hayro/hayro-svg/examples/rendered_0.svg");

    virtual_scroll(0..1, move |state, index| {
        flex_col((
            render_svg_to_canvas(svg2, 200.0),
            render_svg_to_canvas(svg1, 400.0),
        ))
        .padding(Padding::from(200.0))
    })
}
