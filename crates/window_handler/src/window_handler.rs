use app;

use log::{error, info};
use winit::event_loop::ControlFlow;
use winit::window::WindowBuilder;

/*
enum CustomEvent {
    RequestRedraw
}

struct ExampleRepaintSignal(std::sync::Mutex<winit::event_loop::EventLoopProxy<CustomEvent>>);

impl epi::backend::RepaintSignal for ExampleRepaintSignal {
    fn request_repaint(&self) {
        self.0.lock().unwrap().send_event(CustomEvent::RequestRedraw).ok();
    }
}
*/

pub async fn run() {
    env_logger::init(); // Necessary for logging within WGPU

    let event_loop = winit::event_loop::EventLoopBuilder::<()>::with_user_event()
        .build()
        .unwrap();

    let window = WindowBuilder::new()
        .with_decorations(true)
        .with_resizable(true)
        .with_transparent(false)
        .with_title("egui-wgpu")
        .with_inner_size(winit::dpi::PhysicalSize {
            width: app::INITIAL_WIDTH,
            height: app::INITIAL_HEIGHT,
        })
        .build(&event_loop)
        .unwrap();

    // Opens the window and starts processing events (although no events are handled yet)
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = app::App::new(&window).await;

    match event_loop.run(move |event, elwt| {
        app.input(&event, elwt);
    }) {
        Ok(_) => info!("Event loop exited successfully"),
        Err(e) => error!("Event loop exited with error: {}", e),
    };
}
