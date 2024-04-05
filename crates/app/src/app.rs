mod state;
use std::collections::HashMap;

use state::WindowState;
use ui;

use egui_wgpu_backend::RenderPass;
use log::info;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoopWindowTarget,
    keyboard::KeyCode,
    window::{Window, WindowId},
};

pub const INITIAL_WIDTH: u32 = 1280;
pub const INITIAL_HEIGHT: u32 = 720;

pub struct App<'a> {
    /// Custom cursors assets.

    //custom_cursors: Vec<CursorIcon>,
    /// Application icon.
    //icon: Icon,
    windows: HashMap<WindowId, WindowState<'a>>,
}

impl<'a> App<'a> {
    pub async fn new(window: &'a Window) -> Self {
        let mut w: HashMap<WindowId, WindowState<'a>> = HashMap::new();
        let w_state = match WindowState::new(window).await {
            Ok(w) => w,
            Err(e) => {
                panic!("Failed to create window state: {:?}", e);
            }
        };

        w.insert(window.id(), w_state);

        Self { windows: w }
    }

    pub fn window(&mut self, window_id: WindowId) -> &mut WindowState<'a> {
        self.windows.get_mut(&window_id).expect("Window not found")
    }

    pub fn windows(&mut self) -> &mut HashMap<WindowId, WindowState<'a>> {
        &mut self.windows
    }

    pub fn get_gui(&mut self, window_id: WindowId) -> &mut ui::UI {
        self.windows
            .get_mut(&window_id)
            .expect("Window not found")
            .get_gui()
    }

    pub fn resize(&mut self, window_id: WindowId, new_size: winit::dpi::PhysicalSize<u32>) {
        let window = self.window(window_id);
        if new_size.width > 0 && new_size.height > 0 {
            window.change_config_size(new_size);
            //window.resize(new_size);
            window.window().request_redraw();
        }
    }

    pub fn scale_factor_changed(&mut self, scale_factor: f64) {
        //self.ui.set_scale_factor(scale_factor as f64);
    }

    pub fn input(&mut self, window_id: WindowId, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == winit::event::ElementState::Pressed {
                    if event.physical_key == KeyCode::Escape {
                        drop(WindowEvent::CloseRequested);
                        return true;
                    }
                }
                false
            }
            WindowEvent::CursorMoved { position, .. } => {
                info!("Mouse moved to {:?}", position);
                let window: &mut WindowState = self.window(window_id);
                window.set_clear_color(wgpu::Color {
                    r: position.x / window.get_size().width as f64,
                    g: position.y / window.get_size().height as f64,
                    b: 1.0,
                    a: 1.0,
                });
                window.window().request_redraw();
                true
            }
            _ => false,
        }
    }

    pub fn update(&'a mut self) {}

    pub fn render(&mut self, window_id: WindowId) -> Result<(), wgpu::SurfaceError> {
        for (id, window) in self.windows.iter_mut() {
            if *id == window_id {
                let _ = window.draw();
            }
        }

        Ok(())
    }
}
