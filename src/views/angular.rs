use eframe::egui::{self, Color32, Stroke};
pub struct Angular {
    l: u32,
    m: i32,
}
impl Default for Angular {
    fn default() -> Self {
        Self { l: 1, m: 0 }
    }
}
impl Angular {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.add(egui::Slider::new(&mut self.l, 0..=4).text("l"));
        self.m = self.m.clamp(-(self.l as i32), self.l as i32);
        ui.add(
            egui::Slider::new(&mut self.m, -(self.l as i32)..=self.l as i32).text("m destacado"),
        );
        let magnitude = (self.l as f64 * (self.l + 1) as f64).sqrt();
        ui.strong(format!(
            "|L| = {magnitude:.3} ℏ · Lz = {} ℏ · {} projeções",
            self.m,
            2 * self.l + 1
        ));
        let (rect, _) = ui.allocate_exact_size(
            egui::vec2(ui.available_width(), 320.0),
            egui::Sense::hover(),
        );
        let painter = ui.painter_at(rect);
        painter.rect_filled(rect, 12.0, Color32::from_rgb(237, 244, 250));
        let origin = rect.center() - egui::vec2(80.0, 0.0);
        painter.line_segment(
            [
                origin - egui::vec2(0.0, 140.0),
                origin + egui::vec2(0.0, 140.0),
            ],
            Stroke::new(1.0_f32, Color32::GRAY),
        );
        let scale = 125.0 / magnitude.max(1.0) as f32;
        for m in -(self.l as i32)..=self.l as i32 {
            let horizontal = (magnitude * magnitude - (m * m) as f64).max(0.0).sqrt();
            let end = origin + egui::vec2(horizontal as f32 * scale, -(m as f32) * scale);
            let color = if m == self.m {
                Color32::from_rgb(0, 126, 132)
            } else {
                Color32::from_gray(155)
            };
            painter.arrow(
                origin,
                end - origin,
                Stroke::new(if m == self.m { 3.0_f32 } else { 1.0_f32 }, color),
            );
            painter.text(
                end + egui::vec2(12.0, 0.0),
                egui::Align2::LEFT_CENTER,
                format!("m={m}"),
                egui::FontId::proportional(16.0),
                color,
            );
        }
        ui.label("Esquema de projeções permitidas. As componentes transversais não são valores simultaneamente definidos; as setas não descrevem órbitas.");
        ui.monospace(format!(
            "L² / ℏ² = {} · |L| / ℏ = √{}",
            self.l * (self.l + 1),
            self.l * (self.l + 1)
        ));
    }
}
