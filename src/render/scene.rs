use eframe::egui::{self, Color32, Pos2, Stroke, Vec2};

pub const POSITIVE: Color32 = Color32::from_rgb(37, 112, 196);
pub const NEGATIVE: Color32 = Color32::from_rgb(222, 119, 55);

pub struct Point {
    pub position: [f64; 3],
    pub positive: bool,
}
pub struct Camera {
    yaw: f64,
    pitch: f64,
    zoom: f64,
}
impl Default for Camera {
    fn default() -> Self {
        Self {
            yaw: 0.55,
            pitch: 0.3,
            zoom: 1.0,
        }
    }
}
impl Camera {
    fn rotate(&self, p: [f64; 3]) -> [f64; 3] {
        let x = p[0] * self.yaw.cos() + p[2] * self.yaw.sin();
        let z = -p[0] * self.yaw.sin() + p[2] * self.yaw.cos();
        [
            x,
            p[1] * self.pitch.cos() - z * self.pitch.sin(),
            p[1] * self.pitch.sin() + z * self.pitch.cos(),
        ]
    }
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        id: &str,
        points: &[Point],
        extent: f64,
        unit: &str,
        bounds: Option<[f64; 3]>,
    ) {
        ui.horizontal(|ui| {
            ui.label("Arraste para girar em 3D");
            ui.add(egui::Slider::new(&mut self.zoom, 0.5..=2.5).text("Zoom"));
            if ui.button("Vista inicial").clicked() {
                *self = Self::default();
            }
        });
        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(ui.available_width(), 360.0), egui::Sense::drag());
        if response.dragged() {
            let delta = ui.input(|i| i.pointer.delta());
            self.yaw += delta.x as f64 * 0.01;
            self.pitch = (self.pitch + delta.y as f64 * 0.01).clamp(-1.5, 1.5);
        }
        let painter = ui.painter_at(rect);
        painter.rect_filled(rect, 12.0, Color32::from_rgb(237, 244, 250));
        let scale = rect.height().min(rect.width()) as f64 * 0.40 * self.zoom / extent;
        let center = rect.center();
        let screen = |p: [f64; 3]| {
            Pos2::new(
                center.x + (p[0] * scale) as f32,
                center.y - (p[1] * scale) as f32,
            )
        };
        if let Some(lengths) = bounds {
            for corner in 0..8 {
                let p = std::array::from_fn(|axis| {
                    if corner & (1 << axis) == 0 {
                        -lengths[axis] / 2.0
                    } else {
                        lengths[axis] / 2.0
                    }
                });
                for axis in 0..3 {
                    if corner & (1 << axis) == 0 {
                        let mut other = p;
                        other[axis] += lengths[axis];
                        painter.line_segment(
                            [screen(self.rotate(p)), screen(self.rotate(other))],
                            Stroke::new(1.0_f32, Color32::from_gray(135)),
                        );
                    }
                }
            }
        }
        for (i, label) in ["x", "y", "z"].iter().enumerate() {
            let mut p = [0.0; 3];
            p[i] = extent;
            let end = screen(self.rotate(p));
            painter.line_segment([center, end], Stroke::new(1.0_f32, Color32::from_gray(150)));
            painter.text(
                end,
                egui::Align2::CENTER_CENTER,
                *label,
                egui::FontId::proportional(14.0),
                Color32::from_gray(70),
            );
        }
        let mut rotated: Vec<_> = points
            .iter()
            .map(|p| (self.rotate(p.position), p.positive))
            .collect();
        rotated.sort_by(|a, b| a.0[2].total_cmp(&b.0[2]));
        for (p, positive) in rotated {
            let depth = ((p[2] / extent + 1.0) / 2.0).clamp(0.0, 1.0);
            let color = if positive { POSITIVE } else { NEGATIVE };
            let alpha = (70.0 + depth * 150.0) as u8;
            painter.circle_filled(
                screen(p),
                (if points.len() < 600 { 4.0 } else { 1.6 }) + depth as f32 * 1.2,
                Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha),
            );
        }
        painter.circle_filled(center, 2.0, Color32::from_gray(60));
        painter.text(
            rect.left_bottom() + Vec2::new(12.0, -12.0),
            egui::Align2::LEFT_BOTTOM,
            format!("{id}  ·  escala dos eixos: {extent} {unit}"),
            egui::FontId::proportional(12.0),
            Color32::from_gray(80),
        );
    }
}
