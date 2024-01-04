#[path = "window_handler/window_handler.rs"]
mod window_handler;

use eframe::egui;
use egui::*;

#[derive(Default)]
struct Content {
    text: String,
}

impl Content {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Start with the default fonts (we will be adding to them rather than replacing them).
        let mut fonts = egui::FontDefinitions::default();

        // Install my own font (maybe supporting non-latin characters).
        // .ttf and .otf files supported.
        fonts.font_data.insert(
            "my_font".to_owned(),
            egui::FontData::from_static(include_bytes!("./fonts/Hack-Regular.ttf")),
        );

        // Put my font first (highest priority) for proportional text:
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "my_font".to_owned());

        // Put my font as last fallback for monospace:
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("my_font".to_owned());

        // Tell egui to use these fonts:
        (&cc.egui_ctx).set_fonts(fonts);

        Self {
            text: String::new(),
        }
    }
}

impl eframe::App for Content {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        custom_window_frame(ctx, "egui with custom frame", |ui| {
            ScrollArea::vertical()
                .auto_shrink(false)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    ui.label("This is just the contents of the window.");
                    ui.horizontal(|ui| {
                        ui.label("egui theme:");
                        egui::widgets::global_dark_light_mode_buttons(ui);
                    });

                    ui.heading("Press/Hold/Release example. Press A to test.");

                    if ui.button("Clear").clicked() {
                        self.text.clear();
                    }

                    ScrollArea::vertical()
                        .auto_shrink(false)
                        .stick_to_bottom(true)
                        .show(ui, |ui| {
                            ui.label(&self.text);
                        });

                    if ctx.input(|i| i.key_pressed(Key::A)) {
                        self.text.push_str("\nPressed");
                    }
                    if ctx.input(|i| i.key_down(Key::A)) {
                        self.text.push_str("\nHeld");
                        ui.ctx().request_repaint(); // make sure we note the holding.
                    }
                    if ctx.input(|i| i.key_released(Key::A)) {
                        self.text.push_str("\nReleased");
                    }
                });
        });
    }
}

fn custom_window_frame(ctx: &egui::Context, title: &str, add_contents: impl FnOnce(&mut Ui)) {
    let panel_frame = egui::Frame {
        fill: ctx.style().visuals.window_fill(),
        rounding: 10.0.into(),
        stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
        outer_margin: 0.5.into(),
        ..Default::default()
    };

    CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
        let app_rect = ui.max_rect();

        let title_bar_height = 32.0;
        let title_bar_rect = {
            let mut rect = app_rect;
            rect.max.y = rect.min.y + title_bar_height;
            rect
        };

        title_bar_ui(ui, title_bar_rect, title);

        // Add the contents
        let content_rect = {
            let mut rect = app_rect;
            rect.min.y = title_bar_rect.max.y;
            rect
        }
        .shrink(4.0);

        let mut content_ui = ui.child_ui(content_rect, *ui.layout());
        add_contents(&mut content_ui);
    });
}

fn title_bar_ui(ui: &mut Ui, title_bar_rect: Rect, title: &str) {
    let painter = ui.painter();

    let title_bar_response = ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());

    // Paint the title
    painter.text(
        title_bar_rect.center(),
        Align2::CENTER_CENTER,
        title,
        FontId::proportional(20.0),
        ui.style().visuals.text_color(),
    );

    // Paint the line under the title
    painter.line_segment(
        [
            title_bar_rect.left_bottom() + vec2(1.0, 0.0),
            title_bar_rect.right_bottom() + vec2(-1.0, 0.0),
        ],
        ui.visuals().widgets.noninteractive.bg_stroke,
    );

    // Interact with the title bar (drag to move window)
    if title_bar_response.double_clicked() {
        let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
        ui.ctx()
            .send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
    } else if title_bar_response.is_pointer_button_down_on() {
        let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));

        if is_maximized {
            ui.ctx()
                .send_viewport_cmd(ViewportCommand::Maximized(false));
            // put the window where the mouse is
            let cursor_pos = ui.input(|ui| ui.pointer.interact_pos());
            let cursor_pos = cursor_pos.unwrap_or_default();
            let pointer_pos = title_bar_response.interact_pointer_pos().unwrap_or_default();
            // Get outer position of the window
            let outer_position = ui.ctx().input(|ui| ui.screen_rect());
            let pos = Pos2::new(cursor_pos.x - outer_position.min.x, cursor_pos.y - outer_position.min.y);
            ui.ctx().send_viewport_cmd(ViewportCommand::OuterPosition(pos));
        }

        ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag)
    }

    ui.allocate_ui_at_rect(title_bar_rect, |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
            close_maximize_minimize_buttons(ui);
        });
    });
}

fn close_maximize_minimize_buttons(ui: &mut egui::Ui) {
    let button_height = 12.0;

    let close_response = ui
        .add(Button::new(RichText::new("❌").size(button_height)))
        .on_hover_text("Close the window");

    if close_response.clicked() {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
    }

    let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
    if is_maximized {
        let maximized_response = ui
            .add(Button::new(RichText::new("🗗").size(button_height)))
            .on_hover_text("Restore window");

        if maximized_response.clicked() {
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::Maximized(false));
        }
    } else {
        let maximize_response = ui
            .add(Button::new(RichText::new("🗖").size(button_height)))
            .on_hover_text("Maximize window");

        if maximize_response.clicked() {
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::Maximized(true));
        }
    }

    let minimized_response = ui
        .add(Button::new(RichText::new("➖").size(button_height)))
        .on_hover_text("Minimize window");
    if minimized_response.clicked() {
        ui.ctx()
            .send_viewport_cmd(egui::ViewportCommand::Minimized(true));
    }
}

fn main() {
    //pollster::block_on(window_handler::run());

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_inner_size([1024.0, 720.0])
            .with_min_inner_size([680.0, 360.0])
            .with_transparent(false),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Custom window frame",
        options,
        Box::new(|_cc| Box::<Content>::default()),
    );
}
