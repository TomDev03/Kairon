#[path = "../state/state.rs"]
mod state;

use ::egui::FontDefinitions;
use chrono::Timelike;
use log::{error, info};
use std::iter;
use std::time::Instant;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::{keyboard::KeyCode, window::WindowBuilder};

const INITIAL_WIDTH: u32 = 1920;
const INITIAL_HEIGHT: u32 = 1080;

/*
struct ExampleRepaintSignal(std::sync::Mutex<winit::event_loop::EventLoopProxy<winit::event::Event>>);

impl epi::backend::RepaintSignal for ExampleRepaintSignal {
    fn request_repaint(&self) {
        self.0.lock().unwrap().send_event(Event::RequestRedraw).ok();
    }
}
*/

pub async fn run() {
    env_logger::init(); // Necessary for logging within WGPU

    let event_loop = winit::event_loop::EventLoopBuilder::<()>::with_user_event().build().unwrap();

    let window = WindowBuilder::new()
        .with_decorations(true)
        .with_resizable(true)
        .with_transparent(false)
        .with_title("egui-wgpu")
        .with_inner_size(winit::dpi::PhysicalSize{
            width: INITIAL_WIDTH,
            height: INITIAL_HEIGHT,
        })
        .build(&event_loop)
        .unwrap();

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
                        WindowEvent::ScaleFactorChanged {
                            scale_factor,
                            inner_size_writer,
                            ..
                        } => {
                            // TODO: Handle this error better
                        }
                        _ => (),
                    }
                }
            }
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
