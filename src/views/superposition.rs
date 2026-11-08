use super::{curve, graph};
use eframe::egui;
use iam_lab::physics::{confinement::*, quantum::*};
pub struct Superposition {
    weight: f64,
    phase: f64,
    length: f64,
    playing: bool,
}
impl Default for Superposition {
    fn default() -> Self {
        Self {
            weight: 0.5,
            phase: 0.0,
            length: 1.0,
            playing: false,
        }
    }
}
impl Superposition {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.add(egui::Slider::new(&mut self.weight, 0.0..=1.0).text("Probabilidade w de medir E1"));
        ui.add(egui::Slider::new(&mut self.length, 0.3..=3.0).text("L (nm)"));
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.playing, "Animar fase");
            if ui.button("Voltar a t=0").clicked() {
                self.phase = 0.0;
                self.playing = false;
            }
        });
        if self.playing {
            self.phase = (self.phase + ui.input(|i| i.stable_dt).min(0.1) as f64 * 1.5)
                .rem_euclid(std::f64::consts::TAU);
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_millis(33));
        }
        ui.add(
            egui::Slider::new(&mut self.phase, 0.0..=std::f64::consts::TAU)
                .text("Fase relativa φ (rad)"),
        );
        let e1 = well_energy(1, self.length);
        let e2 = well_energy(2, self.length);
        let delta = e2 - e1;
        graph(
            ui,
            "superposition",
            "x (nm)",
            "densidade (nm^−1)",
            vec![
                (
                    "Superposição coerente",
                    curve(0.0, self.length, |x| {
                        superposition_density(x, self.length, self.weight, self.phase)
                    }),
                ),
                (
                    "Mistura dos mesmos pesos",
                    curve(0.0, self.length, |x| {
                        self.weight * well_wave(1, x, self.length).powi(2)
                            + (1.0 - self.weight) * well_wave(2, x, self.length).powi(2)
                    }),
                ),
            ],
        );
        ui.label(format!(
            "P(E1)={:.0}% · P(E2)={:.0}% · <E>={:.4} eV (constante)",
            self.weight * 100.0,
            (1.0 - self.weight) * 100.0,
            self.weight * e1 + (1.0 - self.weight) * e2
        ));
        ui.label(format!(
            "Transição 2→1: ΔE={delta:.4} eV · λ={:.1} nm · período de interferência={:.3} fs",
            1239.841984 / delta,
            4.135667696 / delta
        ));
        ui.small("Animação desacelerada. Uma volta de fase corresponde a h/ΔE; a velocidade na tela não é o tempo físico. Não simulamos emissão espontânea.");
    }
}
