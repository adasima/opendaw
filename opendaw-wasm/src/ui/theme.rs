use egui::{Color32, Margin, Stroke, Vec2, Visuals, epaint::Shadow, style::WidgetVisuals};

pub fn apply_theme(ctx: &egui::Context) {
    let mut style = (*ctx.global_style()).clone();

    // ==========================================
    // 1. Spacing & Padding (洗練されたパディング)
    // ==========================================
    style.spacing.item_spacing = Vec2::new(10.0, 10.0);
    style.spacing.window_margin = Margin {
        left: 16,
        right: 16,
        top: 16,
        bottom: 16,
    };
    style.spacing.button_padding = Vec2::new(14.0, 8.0);
    style.spacing.menu_margin = Margin {
        left: 8,
        right: 8,
        top: 8,
        bottom: 8,
    };

    // ==========================================
    // 2. Visuals (Ableton-like Dark & Glassmorphism)
    // ==========================================
    let mut visuals = Visuals::dark();

    // カラーパレット設定
    let bg_base = Color32::from_rgb(30, 32, 36);
    let bg_glass = Color32::from_rgba_premultiplied(40, 42, 48, 230); // 半透明（グラスモーフィズム）
    let window_glass = Color32::from_rgba_premultiplied(26, 28, 32, 245);

    let primary_accent = Color32::from_rgb(255, 115, 0); // Ableton的な鮮やかなオレンジ
    let text_main = Color32::from_rgb(220, 220, 220);
    let border_color = Color32::from_rgb(20, 21, 24);

    let widget_bg = Color32::from_rgb(50, 52, 58);
    let widget_hover = Color32::from_rgb(65, 68, 76);
    let widget_active = primary_accent;

    // パネル＆ウィンドウ背景色
    visuals.window_fill = window_glass;
    visuals.panel_fill = bg_glass;
    visuals.faint_bg_color = widget_bg;
    visuals.extreme_bg_color = Color32::from_rgb(18, 19, 21);

    // 選択状態
    visuals.selection.bg_fill = primary_accent;
    visuals.selection.stroke = Stroke::new(1.0, Color32::WHITE);

    // テキストカラーのオーバーライド
    visuals.override_text_color = Some(text_main);

    // ==========================================
    // 3. Shadows (美しいシャドウ)
    // ==========================================
    visuals.window_shadow = Shadow {
        color: Color32::from_black_alpha(140),
        #[allow(clippy::useless_conversion)]
        offset: [0, 4].into(),
        blur: 16,
        spread: 0,
    };
    visuals.popup_shadow = Shadow {
        color: Color32::from_black_alpha(120),
        #[allow(clippy::useless_conversion)]
        offset: [0, 8].into(),
        blur: 24,
        spread: 0,
    };

    // ==========================================
    // 4. Rounding (洗練された角丸)
    // ==========================================
    let common_rounding = 6;
    visuals.window_corner_radius = 12.into();
    visuals.menu_corner_radius = 8.into();

    // ==========================================
    // 5. Widget States (リッチなインタラクション)
    // ==========================================
    visuals.widgets.noninteractive = WidgetVisuals {
        bg_fill: bg_base,
        weak_bg_fill: bg_base,
        bg_stroke: Stroke::new(1.0, border_color),
        corner_radius: common_rounding.into(),
        fg_stroke: Stroke::new(1.0, text_main),
        expansion: 0.0,
    };

    visuals.widgets.inactive = WidgetVisuals {
        bg_fill: widget_bg,
        weak_bg_fill: widget_bg,
        bg_stroke: Stroke::new(1.0, border_color),
        corner_radius: common_rounding.into(),
        fg_stroke: Stroke::new(1.0, text_main),
        expansion: 0.0,
    };

    visuals.widgets.hovered = WidgetVisuals {
        bg_fill: widget_hover,
        weak_bg_fill: widget_hover,
        bg_stroke: Stroke::new(1.0, primary_accent),
        corner_radius: common_rounding.into(),
        fg_stroke: Stroke::new(1.0, Color32::WHITE),
        expansion: 1.0,
    };

    visuals.widgets.active = WidgetVisuals {
        bg_fill: widget_active,
        weak_bg_fill: widget_active,
        bg_stroke: Stroke::new(1.0, Color32::WHITE),
        corner_radius: common_rounding.into(),
        fg_stroke: Stroke::new(1.0, Color32::WHITE),
        expansion: 2.0,
    };

    visuals.widgets.open = WidgetVisuals {
        bg_fill: widget_bg,
        weak_bg_fill: widget_bg,
        bg_stroke: Stroke::new(1.0, border_color),
        corner_radius: common_rounding.into(),
        fg_stroke: Stroke::new(1.0, text_main),
        expansion: 0.0,
    };

    style.visuals = visuals;

    // スタイルを適用
    ctx.set_global_style(style);
}
