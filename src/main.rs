use std::sync::Arc;
use tracing::{debug, info};
use xilem::core::Edit;
use xilem::masonry::properties::types::Length;
use xilem::view::{canvas, sized_box, virtual_scroll};
use xilem::winit::error::EventLoopError;
use xilem::{Affine, EventLoop, EventLoopBuilder, WidgetView, WindowOptions, Xilem};

enum MainState {
    Online,
}

fn main() -> Result<(), EventLoopError> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();
    info!("Starting PDF application");
    run(EventLoop::with_user_event())
}

fn run(event_loop: EventLoopBuilder) -> Result<(), EventLoopError> {
    Xilem::new_simple(
        MainState::Online,
        app,
        WindowOptions::new("A PDF Application"),
    )
    .run_in(event_loop)
}

fn render_svg_to_canvas(svg_str: &'static str) -> impl WidgetView<Edit<MainState>> + use<> {
    debug!("Rendering SVG to canvas");
    // Parse SVG and generate Vello scene
    let svg_scene = vello_svg::render(svg_str).unwrap();
    let tree = usvg::Tree::from_str(svg_str, &usvg::Options::default()).unwrap();
    let svg_size = tree.size();
    debug!("SVG size: {}x{}", svg_size.width(), svg_size.height());

    sized_box(canvas(Arc::new(move |scene, size| {
        tracing::trace!("Canvas draw callback invoked with size: {:?}", size);
        // Compute scaling
        let scale_x = size.width / svg_size.width() as f64;
        // Uniform scaling about SVG center
        let transform =
            Affine::scale_about(scale_x, (svg_size.width() / 2.0, svg_size.height() / 2.0));
        scene.append(&svg_scene, Some(transform));
    })))
    .height(Length::px((svg_size.height() + 400f32) as f64))
}

fn app(_state: &mut MainState) -> impl WidgetView<Edit<MainState>> + use<> {
    info!("Building app view");
    let svg1 = include_str!("../testing_pdfs/rendered_2.svg");
    let svg2 = include_str!("../testing_pdfs/rendered_0.svg");

    virtual_scroll(0..2, move |_state: &mut MainState, index| {
        debug!("Virtual scroll rendering item {}", index);
        let svg = match index {
            0 => svg1,
            1 => svg2,
            _ => unreachable!(),
        };
        render_svg_to_canvas(svg)
    })
}
