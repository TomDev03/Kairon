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
        // if new_size.width > 0 && new_size.height > 0 {
        //     self.size = new_size;
        //     self.config.width = new_size.width;
        //     self.config.height = new_size.height;
        //     self.surface.configure(&self.device, &self.config);

        //     window.request_redraw();
        // }
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

    pub fn update(&'a mut self, event: Event<()>, elwt: &EventLoopWindowTarget<()>) {}

    pub fn render(&mut self, window_id: WindowId) -> Result<(), wgpu::SurfaceError> {
        // let egui_rpass: RenderPass = RenderPass::new(&self.device, self.config.format, 1);

        // self.ui.ui(
        //     window,
        //     &self.device,
        //     &self.queue,
        //     &self.surface,
        //     egui_rpass,
        //     &mut self.config,
        // );

        // let output = self.surface.get_current_texture().unwrap();
        // let view = output
        //     .texture
        //     .create_view(&wgpu::TextureViewDescriptor::default());
        // let mut encoder = self
        //     .device
        //     .create_command_encoder(&wgpu::CommandEncoderDescriptor {
        //         label: Some("Render Encoder"),
        //     });

        // {
        //     let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        //         label: Some("Render Pass"),
        //         color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        //             view: &view,
        //             resolve_target: None,
        //             ops: wgpu::Operations {
        //                 load: wgpu::LoadOp::Clear(self.clear_color),
        //                 store: StoreOp::Store,
        //             },
        //         })],
        //         timestamp_writes: None,
        //         occlusion_query_set: None,
        //         depth_stencil_attachment: None,
        //     });
        // }

        // self.queue.submit(std::iter::once(encoder.finish()));
        // output.present();

        Ok(())
    }
}
