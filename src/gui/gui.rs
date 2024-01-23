#[path = "../egui_winit_platform/egui_winit_platform.rs"]
mod egui_winit_platform;

use std::time::Instant;

use winit::event::Event;
use ::egui::FontDefinitions;
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use wgpu::{Device, SurfaceConfiguration, Surface};
use wgpu::TextureViewDescriptor;

use self::egui_winit_platform::{Platform, PlatformDescriptor};

pub(crate) struct GUI {
    platform: Platform,
    start_time: Instant,
    scale_factor : f64,
    demo_app: egui_demo_lib::DemoWindows,
}

impl GUI {

    pub fn new(scale_factor: f64, width : u32, height : u32) -> Self {
        let platform = Platform::new(PlatformDescriptor {
            physical_width: width,
            physical_height: height,
            scale_factor: scale_factor,
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });

        let mut demo_app = egui_demo_lib::DemoWindows::default();

        Self {
            platform,
            start_time: Instant::now(),
            scale_factor: scale_factor,
            demo_app,
        }
    }

    pub fn set_scale_factor(&mut self, scale_factor: f64) {
        self.scale_factor = scale_factor;
    }

    pub fn handle_event(&mut self, event: &Event<()>) {
        self.platform.handle_event(event);
    }

    pub fn ui(&mut self, device: &Device, surface: Surface, surface_config: &SurfaceConfiguration) {

        self.platform.update_time(self.start_time.elapsed().as_secs_f64());

        let output_frame = match surface.get_current_texture() {
            Ok(frame) => frame,
            Err(wgpu::SurfaceError::Outdated) => {
                // This error occurs when the app is minimized on Windows.
                // Silently return here to prevent spamming the console with:
                // "The underlying surface has changed, and therefore the swap chain must be updated"
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

    }

}