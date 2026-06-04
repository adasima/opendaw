#[allow(deprecated)]
use egui::{Align, Context, Frame, Layout, Sense, TopBottomPanel, ViewportCommand};

pub struct TitleBar<'a> {
    title: &'a str,
}

impl<'a> TitleBar<'a> {
    pub fn new(title: &'a str) -> Self {
        Self { title }
    }

    pub fn show(&self, ctx: &Context) {
        let title_bar_height = 32.0;

        let frame = Frame::new().fill(ctx.global_style().visuals.window_fill());

        #[allow(deprecated)]
        TopBottomPanel::top("custom_title_bar")
            .frame(frame)
            .exact_size(title_bar_height)
            .show(ctx, |ui| {
                let title_bar_rect = {
                    let mut rect = ui.max_rect();
                    rect.max.y = rect.min.y + title_bar_height;
                    rect
                };

                let title_bar_response = ui.interact(
                    title_bar_rect,
                    ui.id().with("title_bar"),
                    Sense::click_and_drag(),
                );

                // ドラッグでウィンドウを移動
                if title_bar_response.is_pointer_button_down_on() {
                    ctx.send_viewport_cmd(ViewportCommand::StartDrag);
                }

                // タイトルとコントロールボタンの配置
                ui.horizontal_centered(|ui| {
                    ui.add_space(8.0);
                    ui.heading(self.title);

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.add_space(8.0);

                        let button_size = egui::vec2(24.0, 24.0);

                        // 閉じるボタン
                        if ui
                            .add(egui::Button::new("❌").min_size(button_size))
                            .clicked()
                        {
                            ctx.send_viewport_cmd(ViewportCommand::Close);
                        }

                        // 最大化・縮小ボタン
                        let is_maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
                        let maximize_icon = if is_maximized { "🗗" } else { "🗖" };
                        if ui
                            .add(egui::Button::new(maximize_icon).min_size(button_size))
                            .clicked()
                        {
                            ctx.send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
                        }

                        // 最小化ボタン
                        if ui
                            .add(egui::Button::new("🗕").min_size(button_size))
                            .clicked()
                        {
                            ctx.send_viewport_cmd(ViewportCommand::Minimized(true));
                        }
                    });
                });
            });
    }
}
