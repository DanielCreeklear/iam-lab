use super::{curve, graph};
use eframe::egui;
use iam_lab::physics::bonding::*;

pub struct Bonding {
    distance: f64,
    depth: f64,
    width: f64,
    electrons_bond: u32,
    electrons_anti: u32,
}
impl Default for Bonding {
    fn default() -> Self {
        Self {
            distance: 1.4,
            depth: 4.5,
            width: 1.5,
            electrons_bond: 2,
            electrons_anti: 0,
        }
    }
}
impl Bonding {
    pub fn valence(&mut self, ui: &mut egui::Ui) {
        ui.label("Ligação de valência: sobreposição de orbitais e emparelhamento de elétrons. Explore como a proximidade favorece a sobreposição, mas a repulsão limita a ligação.");
        ui.add(egui::Slider::new(&mut self.distance, 0.3..=5.0).text("R (a₀)"));
        ui.add(egui::Slider::new(&mut self.depth, 0.5..=8.0).text("Profundidade D (eV)"));
        ui.add(egui::Slider::new(&mut self.width, 0.5..=2.0).text("a (a₀⁻¹)"));
        ui.monospace("S(R) = exp(−R)(1+R+R²/3)\nV(R) = D[(1−exp(−a(R−Re)))²−1], Re=1.4 a₀");
        ui.label(format!(
            "Sobreposição S = {:.4} · V(R) = {:.3} eV",
            overlap(self.distance),
            morse(self.distance, self.depth, self.width, 1.4)
        ));
        graph(
            ui,
            "morse",
            "R (a₀)",
            "V (eV)",
            vec![
                (
                    "Morse",
                    curve(0.5, 5.0, |r| morse(r, self.depth, self.width, 1.4)),
                ),
                (
                    "R selecionado",
                    vec![[self.distance, -self.depth], [self.distance, 5.0]],
                ),
            ],
        );
        graph(
            ui,
            "overlap",
            "R (a₀)",
            "S (adimensional)",
            vec![("Sobreposição 1s–1s", curve(0.0, 6.0, overlap))],
        );
        ui.label("Experimente: aumente a profundidade. O que muda na energia necessária para dissociar a molécula?");
        ui.small("Morse é um modelo fenomenológico independente de S, não um cálculo de Heitler–London. Este laboratório ilustra a ligação de valência; não resolve funções de dois elétrons ou hibridização.");
    }
    pub fn orbitals(&mut self, ui: &mut egui::Ui) {
        ui.label("Combinação linear de dois orbitais atômicos 1s (LCAO). Os sinais representam a fase da função de onda.");
        ui.add(egui::Slider::new(&mut self.distance, 0.3..=5.0).text("Separação R (a₀)"));
        ui.monospace("ψ± = (φA ± φB)/√(2(1±S))");
        graph(
            ui,
            "lcao",
            "x no eixo internuclear (a₀)",
            "ψ (a0^(-3/2))",
            vec![
                (
                    "Ligante σ",
                    curve(-6.0, 6.0, |x| molecular_wave(x, self.distance, false)),
                ),
                (
                    "Antiligante σ*",
                    curve(-6.0, 6.0, |x| molecular_wave(x, self.distance, true)),
                ),
            ],
        );
        ui.add(egui::Slider::new(&mut self.electrons_bond, 0..=2).text("Elétrons em σ"));
        ui.add(egui::Slider::new(&mut self.electrons_anti, 0..=2).text("Elétrons em σ*"));
        let order = (self.electrons_bond as f64 - self.electrons_anti as f64) / 2.0;
        ui.label(format!("Ordem de ligação = (Nσ − Nσ*)/2 = {order:.1}"));
        ui.label("Experimente: compare H₂ (2,0) e He₂ (2,2) neste modelo. Localize o nó do orbital antiligante.");
        ui.small("Modelo mínimo de dois orbitais. Ocupações são escolhidas livremente para estudo e podem representar estados excitados. Corte axial de funções 3D; a integral do corte não é 1. Sem orbitais p, otimização variacional ou energias ab initio.");
    }
}
