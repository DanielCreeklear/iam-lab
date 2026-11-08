use eframe::egui::{self, Color32, RichText};
pub const NAVY: Color32 = Color32::from_rgb(24, 48, 76);
pub const TEAL: Color32 = Color32::from_rgb(0, 126, 132);
pub const MUTED: Color32 = Color32::from_rgb(83, 103, 122);

pub fn apply(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "noto-math".into(),
        egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/NotoSansMath-Regular.ttf"
        ))
        .into(),
    );
    for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
        fonts
            .families
            .entry(family)
            .or_default()
            .push("noto-math".into());
    }
    ctx.set_fonts(fonts);
    ctx.set_theme(egui::Theme::Light);
    let mut style = (*ctx.style()).clone();
    style
        .text_styles
        .insert(egui::TextStyle::Body, egui::FontId::proportional(16.0));
    style
        .text_styles
        .insert(egui::TextStyle::Button, egui::FontId::proportional(15.0));
    style
        .text_styles
        .insert(egui::TextStyle::Heading, egui::FontId::proportional(27.0));
    style
        .text_styles
        .insert(egui::TextStyle::Small, egui::FontId::proportional(13.0));
    style.spacing.item_spacing = egui::vec2(10.0, 9.0);
    style.spacing.button_padding = egui::vec2(12.0, 8.0);
    style.visuals.panel_fill = Color32::from_rgb(247, 249, 252);
    style.visuals.override_text_color = Some(NAVY);
    style.visuals.selection.bg_fill = Color32::from_rgb(210, 235, 239);
    style.visuals.selection.stroke = egui::Stroke::new(1.0_f32, TEAL);
    ctx.set_style(style);
}
pub fn card(ui: &mut egui::Ui, body: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::new()
        .fill(Color32::WHITE)
        .stroke(egui::Stroke::new(1.0_f32, Color32::from_rgb(222, 230, 238)))
        .corner_radius(12)
        .inner_margin(18)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            body(ui);
        });
}
pub fn callout(ui: &mut egui::Ui, title: &str, text: &str) {
    egui::Frame::new()
        .fill(Color32::from_rgb(229, 242, 244))
        .corner_radius(10)
        .inner_margin(16)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(RichText::new(title).strong().color(TEAL));
            ui.label(text);
        });
}
