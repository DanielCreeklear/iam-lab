use super::{curve, graph};
use eframe::egui;
use iam_lab::physics::spin::*;
pub struct Spin {
    speed: f64,
    gradient: f64,
    field: f64,
    l: u32,
    coupling: f64,
}
impl Default for Spin {
    fn default() -> Self {
        Self {
            speed: 2000.0,
            gradient: 100.0,
            field: 0.2,
            l: 1,
            coupling: 1.0,
        }
    }
}
impl Spin {
    pub fn stern_gerlach(&mut self, ui: &mut egui::Ui) {
        ui.add(
            egui::Slider::new(&mut self.speed, 1000.0..=200_000.0)
                .logarithmic(true)
                .text("Velocidade (m/s)"),
        );
        ui.add(egui::Slider::new(&mut self.gradient, 0.0..=200.0).text("Gradiente dB/dz (T/m)"));
        ui.horizontal(|ui| {
            if ui.button("Exemplo: 2.000 m/s").clicked() {
                self.speed = 2000.0;
                self.gradient = 100.0;
            }
            if ui.button("Dobrar velocidade").clicked() {
                self.speed = (2.0 * self.speed).min(200_000.0);
            }
        });
        graph(
            ui,
            "stern-gerlach",
            "distância x (m)",
            "desvio z (mm)",
            vec![
                (
                    "ms=+1/2, μz=−μB",
                    curve(0.0, 0.4, |x| {
                        1000.0 * stern_gerlach_z(x, self.speed, self.gradient, 0.2)
                    }),
                ),
                (
                    "ms=−1/2, μz=+μB",
                    curve(0.0, 0.4, |x| {
                        -1000.0 * stern_gerlach_z(x, self.speed, self.gradient, 0.2)
                    }),
                ),
            ],
        );
        let separation = 2.0 * stern_gerlach_z(0.4, self.speed, self.gradient, 0.2).abs();
        ui.strong(format!(
            "Separação no detector: {:.6} mm",
            separation * 1000.0
        ));
        ui.label("Ímã: x=0 a 0,20 m. Deriva livre: 0,20 a 0,40 m. Após o ímã, as trajetórias tornam-se retas.");
        ui.label("Populações ideais: 50% em cada feixe, para entrada não polarizada. As curvas mostram centros de feixes.");
        ui.small("Átomos H(1s), s=1/2. Sem spin nuclear, colisões, largura do feixe ou deflexão longitudinal. Eixos com escalas diferentes; força de gradiente constante dentro do ímã.");
    }
    pub fn zeeman(&mut self, ui: &mut egui::Ui) {
        ui.add(egui::Slider::new(&mut self.l, 0..=3).text("l (s=1/2)"));
        ui.add(
            egui::Slider::new(&mut self.coupling, 0.5..=3.0)
                .text("Coeficiente ξ de spin–órbita (meV)"),
        );
        ui.add(egui::Slider::new(&mut self.field, 0.0..=0.5).text("Campo B (T)"));
        let groups = if self.l == 0 {
            vec![0.5]
        } else {
            vec![self.l as f64 - 0.5, self.l as f64 + 0.5]
        };
        let mut owned = Vec::new();
        for j in groups {
            let factor = spin_orbit_factor(self.l, j);
            ui.label(format!(
                "j={j:.1} · gJ={:.4} · <L·S>/ℏ²={factor:.2} · {} componentes",
                lande(self.l, j),
                (2.0 * j + 1.0) as u32
            ));
            for index in 0..=(2.0 * j) as u32 {
                let mj = -j + index as f64;
                let label = format!("j={j:.1}, mj={mj:+.1}");
                let points = curve(0.0, 0.5, |b| {
                    self.coupling * factor + 1000.0 * zeeman_shift(self.l, j, mj, b)
                });
                owned.push((label, points));
            }
        }
        let mut lines: Vec<_> = owned
            .iter()
            .map(|(name, points)| (name.as_str(), points.clone()))
            .collect();
        lines.push((
            "B selecionado",
            vec![
                [self.field, -self.coupling * 2.0 - 0.2],
                [self.field, self.coupling * 1.5 + 0.2],
            ],
        ));
        graph(ui, "zeeman", "B (T)", "energia relativa (meV)", lines);
        ui.strong("Energias no campo selecionado");
        for j in if self.l == 0 {
            vec![0.5]
        } else {
            vec![self.l as f64 - 0.5, self.l as f64 + 0.5]
        } {
            let values: Vec<_> = (0..=(2.0 * j) as u32)
                .map(|i| {
                    let mj = -j + i as f64;
                    format!(
                        "mj={mj:+.1}: {:.5}",
                        self.coupling * spin_orbit_factor(self.l, j)
                            + 1000.0 * zeeman_shift(self.l, j, mj, self.field)
                    )
                })
                .collect();
            ui.label(format!("j={j:.1} → {} meV", values.join(" | ")));
        }
        ui.small("Hso=ξ L·S/ℏ² com ξ ajustável, não a estrutura fina completa do H. g_s=2. Para l>0, μB B≪ξ no domínio permitido; desconsidera Paschen–Back e estrutura hiperfina.");
    }
}
