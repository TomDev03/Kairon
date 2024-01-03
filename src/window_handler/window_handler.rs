#[path = "../state/state.rs"]
mod state;
use log::{error, info};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::KeyCode,
    window::WindowBuilder,
};

pub async fn run() {
    env_logger::init(); // Necessary for logging within WGPU
    let event_loop = match EventLoop::new() {
        Ok(el) => el,
        Err(e) => {
            // TODO: Handle this error better
            // check EventLoopErro enum for more info on how to handle the multiple errors more specifically
            error!("Failed to create event loop: {}", e);
            panic!("Failed to start application");
        }
    }; // Loop provided by winit for handling window events

    let window = match WindowBuilder::new().build(&event_loop) {
        Ok(w) => w,
        Err(e) => {
            error!("Failed to create window: {}", e);
            panic!("Failed to start application");
        }
    }; // Window provided by winit for creating a window

    // Opens the window and starts processing events (although no events are handled yet)
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut state = state::State::new(window).await;

    match event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::RedrawRequested => {
                            state.update();
                            match state.render() {
                                Ok(_) => (),
                                // Reconfigure the surface if lost
                                Err(wgpu::SurfaceError::Lost) => state.resize(state.get_size()),
                                // The system is out of memory, we should probably quit
                                Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                                // All other errors (Outdated, Timeout) should be resolved by the next frame
                                Err(e) => eprintln!("{:?}", e),
                            }
                        }
                        WindowEvent::CloseRequested => elwt.exit(),
                        WindowEvent::KeyboardInput { event, .. } => {
                            if event.physical_key == KeyCode::Escape {
                                elwt.exit();
                            }
                        }
                        WindowEvent::Resized(resized) => {
                            state.resize(*resized);
                        }
                        WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer, .. } => {
                            // TODO: Handle this error better
                        }
                        _ => (),
                    }
                }
            },
            Event::AboutToWait => {
                //state.window().request_redraw();
            }
            _ => (),
        }
    }) {
        Ok(_) => info!("Event loop exited successfully"),
        Err(e) => {
            // TODO: Handle this error better
            // check EventLoopErro enum for more info on how to handle the multiple errors more specifically
            error!("Event loop exited with error: {}", e)
        }
    };
}
