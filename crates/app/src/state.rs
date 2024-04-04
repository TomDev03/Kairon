use core::num::NonZeroU32;
use log::error;
use std::error::Error;
use std::mem;
use wgpu::Surface;
use winit::dpi::{LogicalSize, PhysicalPosition, PhysicalSize};
use winit::keyboard::ModifiersState;
use winit::window::Window;
use winit::window::{CursorGrabMode, Fullscreen, ResizeDirection, Theme};

/// The amount of points to around the window for drag resize direction calculations.
const BORDER_SIZE: f64 = 20.;

/// State of the window.
pub struct WindowState<'a> {
    /// IME input.
    ime: bool,
    /// Render surface.
    ///
    /// NOTE: This surface must be dropped before the `Window`.
    surface: Surface<'a>,
    // Surface configuration.
    surface_config: wgpu::SurfaceConfiguration,
    /// The actual winit Window.
    window: &'a Window,
    /// The wgpu instance.
    device: wgpu::Device,
    /// The wgpu queue.
    queue: wgpu::Queue,
    /// UI of the window
    ui: ui::UI,
    /// The window theme we're drawing with.
    theme: Theme,
    /// Cursor position over the window.
    cursor_position: Option<PhysicalPosition<f64>>,
    /// Window modifiers state.
    modifiers: ModifiersState,
    /// Occlusion state of the window.
    occluded: bool,
    /// Current cursor grab mode.
    cursor_grab: CursorGrabMode,
    /// The amount of zoom into window.
    zoom: f64,
    /// The amount of rotation of the window.
    rotated: f32,
    // Size of the window.
    size: PhysicalSize<u32>,

    #[cfg(macos_platform)]
    option_as_alt: OptionAsAlt,

    // Window clear color.
    clear_color: wgpu::Color,

    // Cursor states.
    named_idx: usize,
    //custom_idx: usize,
    cursor_hidden: bool,
}

impl<'a> WindowState<'a> {
    pub async fn new(window: &'a Window) -> Result<Self, Box<dyn Error>> {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

        let surface = match unsafe { instance.create_surface(window) } {
            Ok(s) => s,
            Err(e) => {
                // TODO: Handle this error better
                // check CreateSurfaceError enum for more info on how to handle the multiple errors more specifically
                error!("Failed to create surface: {}", e);
                panic!("Failed to start application");
            }
        };

        let theme = window.theme().unwrap_or(Theme::Dark);
        println!("Theme: {theme:?}");
        let named_idx = 0;
        //window.set_cursor_icon(CURSORS[named_idx]);

        let ui = ui::UI::new(window.scale_factor(), size.width, size.height);

        let clear_color = wgpu::Color::BLACK;

        // Allow IME out of the box.
        let ime = true;
        window.set_ime_allowed(ime);

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

        let mut state = Self {
            #[cfg(macos_platform)]
            option_as_alt: window.option_as_alt(),
            //custom_idx: app.custom_cursors.len() - 1,
            cursor_grab: CursorGrabMode::None,
            named_idx,
            surface,
            surface_config,
            device,
            queue,
            window: &window,
            ui,
            clear_color,
            theme,
            ime,
            size,
            cursor_position: Default::default(),
            cursor_hidden: Default::default(),
            modifiers: Default::default(),
            occluded: Default::default(),
            rotated: Default::default(),
            zoom: Default::default(),
        };

        state.resize(size);
        Ok(state)
    }

    pub fn window(&self) -> &Window {
        self.window
    }

    pub fn set_clear_color(&mut self, color: wgpu::Color) {
        self.clear_color = color;
    }

    pub fn get_size(&self) -> PhysicalSize<u32> {
        self.size
    }

    pub fn get_gui(&mut self) -> &mut ui::UI {
        &mut self.ui
    }

    pub fn toggle_ime(&mut self) {
        self.ime = !self.ime;
        self.window.set_ime_allowed(self.ime);
        if let Some(position) = self.ime.then_some(self.cursor_position).flatten() {
            self.window
                .set_ime_cursor_area(position, PhysicalSize::new(20, 20));
        }
    }

    pub fn minimize(&mut self) {
        self.window.set_minimized(true);
    }

    pub fn cursor_moved(&mut self, position: PhysicalPosition<f64>) {
        self.cursor_position = Some(position);
        if self.ime {
            self.window
                .set_ime_cursor_area(position, PhysicalSize::new(20, 20));
        }
    }

    pub fn cursor_left(&mut self) {
        self.cursor_position = None;
    }

    /// Toggle maximized.
    fn toggle_maximize(&self) {
        let maximized = self.window.is_maximized();
        self.window.set_maximized(!maximized);
    }

    /// Toggle window decorations.
    fn toggle_decorations(&self) {
        let decorated = self.window.is_decorated();
        self.window.set_decorations(!decorated);
    }

    /// Toggle window resizable state.
    fn toggle_resizable(&self) {
        let resizable = self.window.is_resizable();
        self.window.set_resizable(!resizable);
    }

    /// Toggle cursor visibility
    fn toggle_cursor_visibility(&mut self) {
        self.cursor_hidden = !self.cursor_hidden;
        self.window.set_cursor_visible(!self.cursor_hidden);
    }

    /// Toggle resize increments on a window.
    fn toggle_resize_increments(&mut self) {
        let new_increments = match self.window.resize_increments() {
            Some(_) => None,
            None => Some(LogicalSize::new(25.0, 25.0)),
        };
        println!("Had increments: {}", new_increments.is_none());
        self.window.set_resize_increments(new_increments);
    }

    /// Toggle fullscreen.
    fn toggle_fullscreen(&self) {
        let fullscreen = if self.window.fullscreen().is_some() {
            None
        } else {
            Some(Fullscreen::Borderless(None))
        };

        self.window.set_fullscreen(fullscreen);
    }

    /// Cycle through the grab modes ignoring errors.
    fn cycle_cursor_grab(&mut self) {
        self.cursor_grab = match self.cursor_grab {
            CursorGrabMode::None => CursorGrabMode::Confined,
            CursorGrabMode::Confined => CursorGrabMode::Locked,
            CursorGrabMode::Locked => CursorGrabMode::None,
        };
        println!("Changing cursor grab mode to {:?}", self.cursor_grab);
        if let Err(err) = self.window.set_cursor_grab(self.cursor_grab) {
            eprintln!("Error setting cursor grab: {err}");
        }
    }

    #[cfg(macos_platform)]
    fn cycle_option_as_alt(&mut self) {
        self.option_as_alt = match self.option_as_alt {
            OptionAsAlt::None => OptionAsAlt::OnlyLeft,
            OptionAsAlt::OnlyLeft => OptionAsAlt::OnlyRight,
            OptionAsAlt::OnlyRight => OptionAsAlt::Both,
            OptionAsAlt::Both => OptionAsAlt::None,
        };
        println!("Setting option as alt {:?}", self.option_as_alt);
        self.window.set_option_as_alt(self.option_as_alt);
    }

    /// Swap the window dimensions with `request_inner_size`.
    fn swap_dimensions(&mut self) {
        let old_inner_size = self.window.inner_size();
        let mut inner_size = old_inner_size;

        mem::swap(&mut inner_size.width, &mut inner_size.height);
        println!("Requesting resize from {old_inner_size:?} to {inner_size:?}");

        if let Some(new_inner_size) = self.window.request_inner_size(inner_size) {
            if old_inner_size == new_inner_size {
                println!("Inner size change got ignored");
            } else {
                self.resize(new_inner_size);
            }
        } else {
            println!("Request inner size is asynchronous");
        }
    }

    /// Pick the next cursor.
    // fn next_cursor(&mut self) {
    //     self.named_idx = (self.named_idx + 1) % CURSORS.len();
    //     println!("Setting cursor to \"{:?}\"", CURSORS[self.named_idx]);
    //     self.window
    //         .set_cursor(Cursor::Icon(CURSORS[self.named_idx]));
    // }

    /// Pick the next custom cursor.
    // fn next_custom_cursor(&mut self, custom_cursors: &[CustomCursor]) {
    //     self.custom_idx = (self.custom_idx + 1) % custom_cursors.len();
    //     let cursor = Cursor::Custom(custom_cursors[self.custom_idx].clone());
    //     self.window.set_cursor(cursor);
    // }

    /// Resize the window to the new size.
    fn resize(&mut self, size: PhysicalSize<u32>) {
        println!("Resized to {size:?}");
        #[cfg(not(any(android_platform, ios_platform)))]
        {
            let (width, height) = match (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
            {
                (Some(width), Some(height)) => (width, height),
                _ => return,
            };
            self.window.request_inner_size(size);
        }
        self.window.request_redraw();
    }

    /// Change the theme.
    fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
        self.window.request_redraw();
    }

    /// Show window menu.
    fn show_menu(&self) {
        if let Some(position) = self.cursor_position {
            self.window.show_window_menu(position);
        }
    }

    /// Drag the window.
    fn drag_window(&self) {
        if let Err(err) = self.window.drag_window() {
            println!("Error starting window drag: {err}");
        } else {
            println!("Dragging window Window={:?}", self.window.id());
        }
    }

    /// Drag-resize the window.
    fn drag_resize_window(&self) {
        let position = match self.cursor_position {
            Some(position) => position,
            None => {
                println!("Drag-resize requires cursor to be inside the window");
                return;
            }
        };

        let win_size = self.window.inner_size();
        let border_size = BORDER_SIZE * self.window.scale_factor();

        let x_direction = if position.x < border_size {
            ResizeDirection::West
        } else if position.x > (win_size.width as f64 - border_size) {
            ResizeDirection::East
        } else {
            // Use arbitrary direction instead of None for simplicity.
            ResizeDirection::SouthEast
        };

        let y_direction = if position.y < border_size {
            ResizeDirection::North
        } else if position.y > (win_size.height as f64 - border_size) {
            ResizeDirection::South
        } else {
            // Use arbitrary direction instead of None for simplicity.
            ResizeDirection::SouthEast
        };

        let direction = match (x_direction, y_direction) {
            (ResizeDirection::West, ResizeDirection::North) => ResizeDirection::NorthWest,
            (ResizeDirection::West, ResizeDirection::South) => ResizeDirection::SouthWest,
            (ResizeDirection::West, _) => ResizeDirection::West,
            (ResizeDirection::East, ResizeDirection::North) => ResizeDirection::NorthEast,
            (ResizeDirection::East, ResizeDirection::South) => ResizeDirection::SouthEast,
            (ResizeDirection::East, _) => ResizeDirection::East,
            (_, ResizeDirection::South) => ResizeDirection::South,
            (_, ResizeDirection::North) => ResizeDirection::North,
            _ => return,
        };

        if let Err(err) = self.window.drag_resize_window(direction) {
            println!("Error starting window drag-resize: {err}");
        } else {
            println!("Drag-resizing window Window={:?}", self.window.id());
        }
    }

    /// Change window occlusion state.
    fn set_occluded(&mut self, occluded: bool) {
        self.occluded = occluded;
        if !occluded {
            self.window.request_redraw();
        }
    }

    /// Draw the window contents.
    fn draw(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
