use super::{curve, graph};
use eframe::egui;
use iam_lab::physics::confinement::oscillator_wave;
pub struct Oscillator {
    n: u32,
    quantum: f64,
}
impl Default for Oscillator {
    fn default() -> Self {
        Self { n: 0, quantum: 0.2 }
    }
}
impl Oscillator {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.add(egui::Slider::new(&mut self.n, 0..=6).text("Estado n"));
        ui.add(egui::Slider::new(&mut self.quantum, 0.05..=1.0).text("ℏω (eV)"));
        ui.strong(format!(
            "En={:.3} eV · E0={:.3} eV · espaçamento={:.3} eV",
            (self.n as f64 + 0.5) * self.quantum,
            0.5 * self.quantum,
            self.quantum
        ));
        let mut lines = vec![("V / ℏω", curve(-4.0, 4.0, |q| q * q / 2.0))];
        let labels: Vec<String> = (0..=6).map(|n| format!("n={n}")).collect();
        for (n, label) in labels.iter().enumerate() {
            lines.push((
                label.as_str(),
                vec![[-3.5, n as f64 + 0.5], [3.5, n as f64 + 0.5]],
            ));
        }
        graph(ui, "oscillator-levels", "q (adimensional)", "E / ℏω", lines);
        graph(
            ui,
            "oscillator-density",
            "q (adimensional)",
            "densidade em q",
            vec![(
                "|ψn(q)|²",
                curve(-5.0, 5.0, |q| oscillator_wave(self.n, q).powi(2)),
            )],
        );
        ui.label(format!(
            "Nós: {} · retorno clássico: q=±{:.2}",
            self.n,
            (2.0 * self.n as f64 + 1.0).sqrt()
        ));
        ui.small("Soluções analíticas de Hermite. O eixo de energia do gráfico é dividido por ℏω; os valores físicos em eV estão acima.");
    }
}
