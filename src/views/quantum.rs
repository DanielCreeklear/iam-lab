use super::{curve, graph};
use eframe::egui;
use iam_lab::physics::quantum::*;

pub struct Quantum {
    n: u32,
    length: f64,
    charge: f64,
}
impl Default for Quantum {
    fn default() -> Self {
        Self {
            n: 1,
            length: 1.0,
            charge: 1.0,
        }
    }
}
impl Quantum {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.label("Confinamento: estados estacionários de um elétron em um poço infinito 1D.");
        ui.monospace("ψₙ(x) = √(2/L) sen(nπx/L)     Eₙ = ℏ²π²n²/(2mL²)");
        ui.add(egui::Slider::new(&mut self.n, 1..=6).text("Número quântico n"));
        ui.add(egui::Slider::new(&mut self.length, 0.2..=3.0).text("L (nm)"));
        ui.label(format!(
            "Energia: {:.4} eV · nós internos: {}",
            well_energy(self.n, self.length),
            self.n - 1
        ));
        graph(
            ui,
            "well",
            "x (nm)",
            "densidade (nm⁻¹)",
            vec![(
                "|ψ|²",
                curve(0.0, self.length, |x| {
                    well_wave(self.n, x, self.length).powi(2)
                }),
            )],
        );
        ui.separator();
        ui.label("Átomo hidrogenoide: um único elétron. Distribuição radial do estado 1s.");
        ui.add(
            egui::Slider::new(&mut self.charge, 1.0..=5.0)
                .integer()
                .text("Carga nuclear Z"),
        );
        ui.monospace("P₁s(r) = 4Z³r² exp(−2Zr) · r em a₀");
        ui.label(format!(
            "E₁ = {:.3} eV · raio mais provável = {:.3} a₀",
            hydrogen_energy(1, self.charge),
            1.0 / self.charge
        ));
        graph(
            ui,
            "radial",
            "r (a₀)",
            "P(r) (a₀⁻¹)",
            vec![(
                "P₁s",
                curve(0.0, 6.0 / self.charge, |r| radial_1s(r, self.charge)),
            )],
        );
        ui.label("Experimente: dobre L. A energia cai por qual fator? Por que P(0)=0 mesmo com densidade eletrônica 1s não nula no núcleo?");
        ui.small("Poço com paredes infinitas; átomo não relativístico, sem spin, blindagem ou correlação multieletrônica.");
    }
}
