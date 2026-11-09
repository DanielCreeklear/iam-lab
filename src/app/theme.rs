use eframe::egui::{self, Color32, RichText};
pub const INK: Color32 = Color32::from_rgb(34, 43, 45);
pub const TEAL: Color32 = Color32::from_rgb(22, 104, 98);
pub const MUTED: Color32 = Color32::from_rgb(83, 93, 90);
pub const PAPER: Color32 = Color32::from_rgb(250, 249, 245);
pub const RAIL: Color32 = Color32::from_rgb(241, 240, 234);

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
    style.spacing.item_spacing = egui::vec2(10.0, 7.0);
    style.spacing.button_padding = egui::vec2(12.0, 8.0);
    style.visuals.panel_fill = PAPER;
    style.visuals.window_fill = PAPER;
    style.visuals.override_text_color = Some(INK);
    style.visuals.selection.bg_fill = Color32::from_rgb(220, 234, 228);
    style.visuals.selection.stroke = egui::Stroke::new(1.0_f32, TEAL);
    style.visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
    style.visuals.widgets.noninteractive.bg_fill = PAPER;
    ctx.set_style(style);
}
pub fn callout(ui: &mut egui::Ui, title: &str, text: &str) {
    ui.label(RichText::new(title).strong().color(TEAL));
    ui.label(text);
}

pub fn section_title(ui: &mut egui::Ui, title: &str) {
    ui.add_space(15.0);
    ui.label(RichText::new(title).strong().color(INK));
    ui.separator();
}
