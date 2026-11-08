use eframe::egui;
use egui_plot::{Plot, Points};
use iam_lab::physics::liquid::Liquid;

pub struct LiquidView {
    model: Liquid,
    temperature: f64,
    density: f64,
    running: bool,
}
impl Default for LiquidView {
    fn default() -> Self {
        Self {
            model: Liquid::new(0.55),
            temperature: 1.0,
            density: 0.55,
            running: false,
        }
    }
}
impl LiquidView {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.label("Fluido de 36 partículas em 2D: amostragem de Monte Carlo com potencial Lennard-Jones e fronteiras periódicas.");
        ui.add(egui::Slider::new(&mut self.temperature, 0.1..=3.0).text("T* = kBT/ε"));
        if ui
            .add(
                egui::Slider::new(&mut self.density, 0.15..=0.85)
                    .text("Densidade Nσ²/A (reinicia)"),
            )
            .changed()
        {
            self.model = Liquid::new(self.density);
        }
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.running, "Executar amostragem");
            if ui.button("10 varreduras").clicked() {
                for _ in 0..10 {
                    self.model.sweep(self.temperature);
                }
            }
            if ui.button("Reiniciar").clicked() {
                self.model = Liquid::new(self.density);
            }
        });
        if self.running {
            for _ in 0..3 {
                self.model.sweep(self.temperature);
            }
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_millis(33));
        }
        let acceptance = self.model.accepted as f64 / self.model.attempted.max(1) as f64;
        ui.label(format!(
            "Tentativas: {} · aceitação: {:.1}% · U/N = {:.3} ε",
            self.model.attempted,
            100.0 * acceptance,
            self.model.energy_per_particle()
        ));
        Plot::new("liquid")
            .height(370.0)
            .data_aspect(1.0)
            .include_x(0.0)
            .include_x(self.model.side)
            .include_y(0.0)
            .include_y(self.model.side)
            .x_axis_label("x/σ")
            .y_axis_label("y/σ")
            .show(ui, |plot| {
                plot.points(
                    Points::new("Partículas", self.model.positions.clone()).radius(7.0_f32),
                );
            });
        ui.monospace("P(aceitar) = min[1, exp(−ΔU/T*)]");
        ui.label("Experimente: diminua T*. Observe a tendência a agrupamento e compare a energia após muitas varreduras.");
        ui.small("Passos de Monte Carlo não são tempo físico. Estado inicial em grade; exige equilibração. Não identifica fases nem mede propriedades macroscópicas. Corte 2.5σ, potencial deslocado; sem correção de cauda. Semente fixa para reprodutibilidade.");
    }
}
