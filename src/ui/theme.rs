use egui::{Color32, FontId, FontFamily, Margin, CornerRadius, Stroke, Style, Vec2, Visuals};

pub struct Theme;

impl Theme {
    // Ableton Live Dark-like Colors
    pub const BG_APP: Color32 = Color32::from_rgb(34, 34, 34);
    pub const BG_PANEL: Color32 = Color32::from_rgb(51, 51, 51);
    pub const BG_HEADER: Color32 = Color32::from_rgb(68, 68, 68);
    pub const BORDER: Color32 = Color32::from_rgb(24, 24, 24);

    pub const TEXT_MUTED: Color32 = Color32::from_rgb(153, 153, 153);
    pub const TEXT_MAIN: Color32 = Color32::from_rgb(204, 204, 204);
    pub const TEXT_ACTIVE: Color32 = Color32::from_rgb(238, 238, 238);

    pub const ACCENT: Color32 = Color32::from_rgb(255, 153, 0); // Live-like orange
    pub const SELECTION_BG: Color32 = Color32::from_rgb(85, 85, 85);

    // Font Sizes
    pub const FONT_SIZE_SMALL: f32 = 10.0;
    pub const FONT_SIZE_BODY: f32 = 12.0;
    pub const FONT_SIZE_HEADING: f32 = 16.0;

    pub fn apply(ctx: &egui::Context) {
        let mut style = (*ctx.global_style()).clone();

        // Define fonts
        for (text_style, font_id) in style.text_styles.iter_mut() {
            match *text_style {
                egui::TextStyle::Small => {
                    *font_id = FontId::new(Self::FONT_SIZE_SMALL, FontFamily::Proportional)
                }
                egui::TextStyle::Body => {
                    *font_id = FontId::new(Self::FONT_SIZE_BODY, FontFamily::Proportional)
                }
                egui::TextStyle::Button => {
                    *font_id = FontId::new(Self::FONT_SIZE_BODY, FontFamily::Proportional)
                }
                egui::TextStyle::Heading => {
                    *font_id = FontId::new(Self::FONT_SIZE_HEADING, FontFamily::Proportional)
                }
                egui::TextStyle::Monospace => {
                    *font_id = FontId::new(Self::FONT_SIZE_BODY, FontFamily::Monospace)
                }
                _ => *font_id = FontId::new(Self::FONT_SIZE_BODY, FontFamily::Proportional),
            }
        }

        // Define visuals
        let mut visuals = Visuals::dark();

        // Backgrounds
        visuals.window_fill = Self::BG_PANEL;
        visuals.window_stroke = Stroke::new(1.0, Self::BORDER);
        visuals.panel_fill = Self::BG_APP;

        // Widget states
        visuals.widgets.noninteractive.bg_fill = Self::BG_APP;
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, Self::BORDER);
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Self::TEXT_MAIN);

        visuals.widgets.inactive.bg_fill = Self::BG_HEADER;
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, Self::BORDER);
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, Self::TEXT_MAIN);

        visuals.widgets.hovered.bg_fill = Self::SELECTION_BG;
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, Self::ACCENT);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Self::TEXT_ACTIVE);

        visuals.widgets.active.bg_fill = Self::ACCENT;
        visuals.widgets.active.bg_stroke = Stroke::new(1.0, Self::ACCENT);
        visuals.widgets.active.fg_stroke = Stroke::new(1.0, Self::TEXT_ACTIVE);

        visuals.selection.bg_fill = Self::SELECTION_BG;
        visuals.selection.stroke = Stroke::new(1.0, Self::ACCENT);

        // Rounding (Live usually has sharp or very slightly rounded corners)
        let rounding = CornerRadius::same(2);
        visuals.widgets.noninteractive.corner_radius = rounding;
        visuals.widgets.inactive.corner_radius = rounding;
        visuals.widgets.hovered.corner_radius = rounding;
        visuals.widgets.active.corner_radius = rounding;
        visuals.window_corner_radius = rounding;
        visuals.menu_corner_radius = rounding;

        style.visuals = visuals;

        // Spacing (Compact for DAW)
        style.spacing.item_spacing = Vec2::new(6.0, 6.0);
        style.spacing.window_margin = Margin::same(6);
        style.spacing.button_padding = Vec2::new(4.0, 2.0);

        ctx.set_global_style(style);
    }
}
