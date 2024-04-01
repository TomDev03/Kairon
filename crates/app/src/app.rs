use ui;

use egui_wgpu_backend::RenderPass;
use log::{error, info};
use wgpu::{StoreOp, SurfaceTarget};
use winit::{event::WindowEvent, keyboard::KeyCode, window::Window};

pub const INITIAL_WIDTH: u32 = 1280;
pub const INITIAL_HEIGHT: u32 = 720;

pub struct App {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    clear_color: wgpu::Color,
    ui: ui::UI,
    window: &'static Window,
}

impl App {
    pub async fn new(window: &'static Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = match instance.create_surface(window) {
            Ok(s) => s,
            Err(e) => {
                // TODO: Handle this error better
                // check CreateSurfaceError enum for more info on how to handle the multiple errors more specifically
                error!("Failed to create surface: {}", e);
                panic!("Failed to start application");
            }
        };

        let adapter = match instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
        {
            Some(a) => a,
            None => {
                error!("Failed to find a suitable adapter");
                panic!("Failed to start application");
            }
        };

        let (device, queue) = match adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::default(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
        {
            Ok((d, q)) => (d, q),
            Err(e) => {
                // TODO: Handle this error better
                // check RequestDeviceError enum for more info on how to handle the multiple errors more specifically
                error!("Failed to create device: {}", e);
                panic!("Failed to start application");
            }
        };

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &surface_config);

        let ui = ui::UI::new(window.scale_factor(), size.width, size.height);

        let clear_color = wgpu::Color::BLACK;

        Self {
            surface,
            device,
            queue,
            config: surface_config,
            size,
            clear_color,
            ui,
            window,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn get_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }

    pub fn get_gui(&mut self) -> &mut ui::UI {
        &mut self.ui
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            self.window.request_redraw();
        }
    }

    pub fn scale_factor_changed(&mut self, scale_factor: f64) {
        self.ui.set_scale_factor(scale_factor as f64);
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
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
                self.clear_color = wgpu::Color {
                    r: position.x / self.size.width as f64,
                    g: position.y / self.size.height as f64,
                    b: 1.0,
                    a: 1.0,
                };
                self.window.request_redraw();
                true
            }
            _ => false,
        }
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let mut egui_rpass: RenderPass = RenderPass::new(&self.device, self.config.format, 1);

        self.ui.ui(
            self.window,
            &self.device,
            &self.queue,
            &self.surface,
            egui_rpass,
            &mut self.config,
        );

        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: StoreOp::Store,
                    },
                })],
                timestamp_writes: None,
                occlusion_query_set: None,
                depth_stencil_attachment: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
