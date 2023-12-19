use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    env_logger::init(); // Necessary for logging within WGPU
    let event_loop = EventLoop::new(); // Loop provided by winit for handling window events

    // Opens the window and starts processing events (although no events are handled yet)
    event_loop.run(move |event, window_target, control_flow| {
        let window = WindowBuilder::new().build(window_target).unwrap();
    });
}
