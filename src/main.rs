use std::sync::Arc;

use tracing::{debug, info};
use vello_svg::vello;
use xilem::{Affine, EventLoop, EventLoopBuilder, WidgetView, WindowOptions, Xilem, core::Edit, masonry::{kurbo::Size, properties::types::Length, widgets::ResizeObserver}, view::{PointerButton, button_any_pointer, canvas, resize_observer, sized_box, transformed, virtual_scroll}, winit::error::EventLoopError};

struct MainState {
	canvas_size: (f64, f64),
	zoom_level:  u16,
	pages:       Vec<Arc<vello::Scene>>,
}

fn main() -> Result<(), EventLoopError> {
	// Initialize tracing
	tracing_subscriber::fmt()
		.with_env_filter(
			tracing_subscriber::EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()),
		)
		.init();

	// begin.
	info!("Starting PDF application");
	run(EventLoop::with_user_event())
}

fn run(event_loop: EventLoopBuilder) -> Result<(), EventLoopError> {
	// Parse the SVGs at startup
	let svg1 = include_str!("../testing_pdfs/rendered_2.svg");
	let svg2 = include_str!("../testing_pdfs/rendered_0.svg");

	let svg_scenes =
		vec![Arc::new(vello_svg::render(svg1).unwrap()), Arc::new(vello_svg::render(svg2).unwrap())];

	Xilem::new_simple(
		MainState { canvas_size: (500f64, 500f64), zoom_level: 1, pages: svg_scenes },
		app,
		WindowOptions::new("A PDF Application"),
	)
	.run_in(event_loop)
}

impl MainState {
	fn render_svg_to_canvas(
		&self,
		svg_str: &'static str,
		svg_scene: Arc<vello::Scene>,
	) -> impl WidgetView<Edit<MainState>> + use<> {
		debug!("Rendering SVG to canvas");
		// Parse SVG to get dimensions
		let tree = usvg::Tree::from_str(svg_str, &usvg::Options::default()).unwrap();

		// Obtain the size for intial rendering canvas
		let svg_size = tree.size();
		debug!("SVG size: {}x{}", svg_size.width(), svg_size.height());

		let (width, height) = self.canvas_size.clone();

		let canvas_view = sized_box(canvas(Arc::new(move |scene, size| {
			tracing::trace!("Canvas draw callback invoked with size: {:?}", size);

			// Compute scaling
			let scale_x = size.width / width;

			// Uniform scaling about SVG center
			let transform = Affine::scale_about(scale_x, (width / 2.0, height / 2.0));

			scene.append(&svg_scene, Some(transform));
		})))
		.height(Length::px(height));

		let with_observer = resize_observer(
			|state: &mut MainState, size| {
				info!("Canvas resized to: {:?}", size);
				state.canvas_size = (size.width, size.height)
			},
			canvas_view,
		);

		transformed(with_observer).transform(Affine::scale(self.zoom_level as f64))
	}
}

fn app(_state: &mut MainState) -> impl WidgetView<Edit<MainState>> + use<> {
	info!("Building app view");
	let svg1 = include_str!("../testing_pdfs/rendered_2.svg");
	let svg2 = include_str!("../testing_pdfs/rendered_0.svg");

	virtual_scroll(0..2, move |state: &mut MainState, index| {
		debug!("Virtual scroll rendering item {}", index);
		let svg = match index {
			0 => svg1,
			1 => svg2,
			_ => unreachable!(),
		};

		// Get the pre-parsed scene for this index
		let svg_scene = state.pages[index as usize].clone();

		button_any_pointer(
			state.render_svg_to_canvas(svg, svg_scene),
			move |state: &mut MainState, button| {
				let button_name = match button {
					None => "Touch/Keyboard".to_string(),
					Some(PointerButton::Primary) => "Left Click".to_string(),
					Some(PointerButton::Secondary) => "Right Click".to_string(),
					Some(PointerButton::Auxiliary) => "Middle Click".to_string(),
					_ => "nah".to_string(),
				};
				state.zoom_level += 1;
				tracing::info!("clicked with: {}", button_name);
			},
		)
	})
}
