use super::{curve, graph};
use eframe::egui;
use iam_lab::physics::{confinement::*, quantum::*};

pub struct Well {
    n: u32,
    length: f64,
    start: f64,
    end: f64,
    mixed: bool,
}
impl Default for Well {
    fn default() -> Self {
        Self {
            n: 1,
            length: 1.0,
            start: 0.0,
            end: 0.5,
            mixed: false,
        }
    }
}
impl Well {
    fn controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.add(egui::Slider::new(&mut self.n, 1..=10).text("Estado n"));
            ui.add(egui::Slider::new(&mut self.length, 0.2..=3.0).text("Largura L (nm)"));
            if ui.button("Restaurar exemplo").clicked() {
                *self = Self::default();
            }
        });
    }
    pub fn eigenstates(&mut self, ui: &mut egui::Ui) {
        self.controls(ui);
        ui.checkbox(&mut self.mixed, "Comparar com superposição de n=1 e n=2");
        let e1 = well_energy(1, self.length);
        let wave = |x| {
            if self.mixed {
                (well_wave(1, x, self.length) + well_wave(2, x, self.length)) / 2.0_f64.sqrt()
            } else {
                well_wave(self.n, x, self.length)
            }
        };
        let operated = |x| {
            if self.mixed {
                (well_wave(1, x, self.length) + 4.0 * well_wave(2, x, self.length)) / 2.0_f64.sqrt()
            } else {
                (self.n * self.n) as f64 * well_wave(self.n, x, self.length)
            }
        };
        graph(
            ui,
            "eigen",
            "x (nm)",
            "amplitude (nm^−1/2)",
            vec![
                ("ψ", curve(0.0, self.length, wave)),
                ("Hψ / E1", curve(0.0, self.length, operated)),
            ],
        );
        ui.label(if self.mixed {"Na superposição, H multiplica as componentes por fatores diferentes. Não existe um único E que devolva Eψ.".into()}else{format!("Hψ = {} E1 × ψ. Energia definida: {:.4} eV.",self.n*self.n,well_energy(self.n,self.length))});
        ui.small(format!("Hψ foi dividido por E1={e1:.4} eV para permitir comparação de amplitudes no mesmo eixo."));
    }
    pub fn probability(&mut self, ui: &mut egui::Ui) {
        self.controls(ui);
        graph(
            ui,
            "well-wave",
            "x (nm)",
            "ψ (nm^−1/2)",
            vec![(
                "ψ: pode ter sinal",
                curve(0.0, self.length, |x| well_wave(self.n, x, self.length)),
            )],
        );
        ui.horizontal_wrapped(|ui| {
            ui.add(egui::Slider::new(&mut self.start, 0.0..=self.end).text("Início a/L"));
            ui.add(egui::Slider::new(&mut self.end, self.start..=1.0).text("Fim b/L"));
        });
        let a = self.start * self.length;
        let b = self.end * self.length;
        egui_plot::Plot::new("well-density")
            .height(230.0)
            .legend(egui_plot::Legend::default())
            .x_axis_label("x (nm)")
            .y_axis_label("|ψ|² (nm^−1)")
            .show(ui, |plot| {
                plot.line(egui_plot::Line::new(
                    "Densidade",
                    curve(0.0, self.length, |x| {
                        well_wave(self.n, x, self.length).powi(2)
                    }),
                ));
                plot.line(
                    egui_plot::Line::new(
                        "Área selecionada",
                        curve(a, b, |x| well_wave(self.n, x, self.length).powi(2)),
                    )
                    .fill(0.0_f32),
                );
            });
        ui.strong(format!(
            "Probabilidade no intervalo: {:.2}%",
            100.0 * interval_probability(self.n, a, b, self.length)
        ));
        ui.label(format!(
            "E = {:.4} eV    <x> = {:.3} nm    <p> = 0    nós internos = {}",
            well_energy(self.n, self.length),
            self.length / 2.0,
            self.n - 1
        ));
        ui.label(format!(
            "Δx Δp / ℏ = {:.4} ≥ 1/2",
            uncertainty_product(self.n)
        ));
        ui.small("Elétron em poço infinito [0,L]. Área calculada analiticamente, não estimada pelos pixels do gráfico.");
    }
}
