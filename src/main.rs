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

fn app(state: &mut MainState) -> impl WidgetView<MainState> + use<> {
    let svg1 = include_str!("/Users/philocalyst/Downloads/hayro/hayro-svg/rendered_0.svg");
    let svg2 = include_str!("/Users/philocalyst/Downloads/hayro/hayro-svg/examples/rendered_0.svg");

    virtual_scroll(0..1, move |state, index| {
        flex_col((
            sized_box(canvas(move |scene, size| {
                // Render the raw SVG → vello scene
                let svg_scene = vello_svg::render(svg2).unwrap();

                // Get the SVG's declared size from the usvg tree instead of vello_svg::render directly
                // (You can tweak your render() to return both Scene and Size)
                let tree = usvg::Tree::from_str(svg2, &usvg::Options::default()).unwrap();
                let svg_size = tree.size();

                // Compute scale to fit
                let scale_x = size.width / svg_size.width() as f64;
                let scale_y = size.height / svg_size.height() as f64;

                // Apply uniform or non-uniform scaling
                let transform = Affine::scale_non_uniform(scale_x, scale_y);

                // Append the scaled tree into your canvas scene
                scene.append(&svg_scene, Some(transform));
            }))
            .height(Length::px(100.0)),
            sized_box(canvas(move |scene, size| {
                // Render the raw SVG → vello scene
                let svg_scene = vello_svg::render(svg1).unwrap();

                // Get the SVG's declared size from the usvg tree instead of vello_svg::render directly
                let tree = usvg::Tree::from_str(svg1, &usvg::Options::default()).unwrap();
                let svg_size = tree.size();

                // Compute scale to fit
                let scale_x = size.width / svg_size.width() as f64;

                // Apply uniform scaling
                let transform =
                    Affine::scale_about(scale_x, (svg_size.width() / 2.0, svg_size.height() / 2.0));

                // Append the scaled tree into your canvas scene
                scene.append(&svg_scene, Some(transform));
            }))
            .height(Length::px(300.0)),
        ))
        .padding(Padding::from(200.0))
    })
}
