use super::{curve, graph};
use eframe::egui;
use iam_lab::physics::interactions::*;

pub struct Interactions {
    a: f64,
    b: f64,
}
impl Default for Interactions {
    fn default() -> Self {
        Self { a: 0.0, b: 0.0 }
    }
}
impl Interactions {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.label("Dipolos permanentes: a orientação muda o sinal da interação elétrica.");
        ui.add(egui::Slider::new(&mut self.a, 0.0..=360.0).text("θ₁ (graus)"));
        ui.add(egui::Slider::new(&mut self.b, 0.0..=360.0).text("θ₂ (graus)"));
        ui.monospace("Udd ∝ [cos(θ₁−θ₂)−3cosθ₁cosθ₂]/r³");
        graph(
            ui,
            "dipoles",
            "r/σ",
            "U (unidade dipolar)",
            vec![(
                "Dipolo–dipolo",
                curve(0.9, 4.0, |r| {
                    dipole_energy(r, self.a.to_radians(), self.b.to_radians())
                }),
            )],
        );
        ui.label("Dispersão e repulsão: potencial de Lennard-Jones.");
        ui.monospace("U/ε = 4[(σ/r)¹²−(σ/r)⁶] · mínimo em r/σ=2^(1/6)");
        graph(
            ui,
            "lj",
            "r/σ",
            "U/ε",
            vec![("Lennard-Jones", curve(0.95, 3.5, lennard_jones))],
        );
        ui.label("Experimente: compare dipolos paralelos alinhados ao eixo e perpendiculares a ele. Quando há atração?");
        ui.small("Dipolos pontuais fixos. LJ é isotrópico; o termo r⁻¹² é empírico. Não representa explicitamente indução, ligações de hidrogênio ou médias orientacionais térmicas.");
    }
}
