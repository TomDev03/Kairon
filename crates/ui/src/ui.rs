use std::iter;
use std::time::Instant;

use ::egui::FontDefinitions;
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use log::error;
use wgpu::TextureViewDescriptor;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use winit::error;
use winit::event::Event;
use winit::window::Window;

pub struct UI {
    platform: Platform,
    start_time: Instant,
    scale_factor: f64,
    demo_app: egui_demo_lib::DemoWindows,
}

impl UI {
    pub fn new(scale_factor: f64, width: u32, height: u32) -> Self {
        let platform = Platform::new(PlatformDescriptor {
            physical_width: width,
            physical_height: height,
            scale_factor,
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });

        let demo_app = egui_demo_lib::DemoWindows::default();

        Self {
            platform,
            start_time: Instant::now(),
            scale_factor,
            demo_app,
        }
    }

    pub fn set_scale_factor(&mut self, scale_factor: f64) {
        self.scale_factor = scale_factor;
    }

    pub fn handle_event(&mut self, event: &Event<()>) {
        self.platform.handle_event(event);
    }

    pub fn ui(
        &mut self,
        window: &Window,
        device: &Device,
        queue: &Queue,
        surface: &Surface,
        mut render_pass: RenderPass,
        surface_config: &SurfaceConfiguration,
    ) {
        self.platform
            .update_time(self.start_time.elapsed().as_secs_f64());

        let output_frame = match surface.get_current_texture() {
            Ok(frame) => frame,
            Err(wgpu::SurfaceError::Outdated) => {
                // This error occurs when the app is minimized on Windows.
                // Silently return here to prevent spamming the console with:
                // "The underlying surface has changed, and therefore the swap chain must be updated"
                error!("Outdated surface error");
                return;
            }
            Err(e) => {
                eprintln!("Dropped frame with error: {}", e);
                return;
            }
        };

        let output_view = output_frame
            .texture
            .create_view(&TextureViewDescriptor::default());

        // Begin to draw the UI frame.
        self.platform.begin_frame();

        // Draw the demo application.
        self.demo_app.ui(&self.platform.context());

        // End the UI frame. We could now handle the output and draw the UI with the backend.
        let full_output = self.platform.end_frame(Some(&window));

        let biding = self
            .platform
            .context()
            .tessellate(full_output.shapes, self.scale_factor as f32);

        let paint_jobs = biding.as_slice();

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("encoder"),
        });

        // Upload all resources for the GPU.
        let screen_descriptor = ScreenDescriptor {
            physical_width: surface_config.width,
            physical_height: surface_config.height,
            scale_factor: self.scale_factor as f32,
        };
        let tdelta: egui::TexturesDelta = full_output.textures_delta as egui::TexturesDelta;

        render_pass
            .add_textures(&device, &queue, &tdelta)
            .expect("add texture ok");

        render_pass.update_buffers(&device, &queue, paint_jobs, &screen_descriptor);

        // Record all render passes.
        match render_pass.execute(
            &mut encoder,
            &output_view,
            &paint_jobs,
            &screen_descriptor,
            Some(wgpu::Color::BLACK),
        ) {
            Ok(_) => {}
            Err(e) => {
                error!("Error: {:?}", e);
            }
        };
        // Submit the commands.
        queue.submit(iter::once(encoder.finish()));

        // Redraw egui
        output_frame.present();

        render_pass
            .remove_textures(tdelta)
            .expect("remove texture ok");
    }
}
