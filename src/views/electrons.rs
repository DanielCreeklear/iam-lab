use eframe::egui;
use iam_lab::physics::electrons::*;
pub struct Electrons {
    z: u32,
}
impl Default for Electrons {
    fn default() -> Self {
        Self { z: 6 }
    }
}
impl Electrons {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        const NAMES: [&str; 18] = [
            "H · Hidrogênio",
            "He · Hélio",
            "Li · Lítio",
            "Be · Berílio",
            "B · Boro",
            "C · Carbono",
            "N · Nitrogênio",
            "O · Oxigênio",
            "F · Flúor",
            "Ne · Neônio",
            "Na · Sódio",
            "Mg · Magnésio",
            "Al · Alumínio",
            "Si · Silício",
            "P · Fósforo",
            "S · Enxofre",
            "Cl · Cloro",
            "Ar · Argônio",
        ];
        ui.add(egui::Slider::new(&mut self.z, 1..=18).text("Número atômico Z (átomo neutro)"));
        ui.heading(NAMES[(self.z - 1) as usize]);
        ui.horizontal_wrapped(|ui| {
            for (name, z) in [
                ("He", 2),
                ("Be", 4),
                ("C", 6),
                ("N", 7),
                ("O", 8),
                ("Ar", 18),
            ] {
                if ui.button(name).clicked() {
                    self.z = z;
                }
            }
        });
        let shells = configuration(self.z);
        ui.add_space(12.0);
        for shell in &shells {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(format!("{}  ({} e−)", shell.label, shell.electrons))
                        .monospace(),
                );
                for state in occupancy(shell.l, shell.electrons) {
                    let text = match state {
                        [true, true] => "↑ ↓",
                        [true, false] => "↑",
                        _ => "—",
                    };
                    egui::Frame::new()
                        .fill(egui::Color32::from_rgb(229, 242, 244))
                        .inner_margin(12)
                        .corner_radius(6)
                        .show(ui, |ui| {
                            ui.set_min_width(34.0);
                            ui.label(egui::RichText::new(text).size(25.0));
                        });
                }
            });
        }
        let config: Vec<_> = shells
            .iter()
            .filter(|s| s.electrons > 0)
            .map(|s| format!("{}^{}", s.label, s.electrons))
            .collect();
        ui.strong(format!("Configuração: {}", config.join(" ")));
        ui.label(format!(
            "Elétrons desemparelhados no diagrama: {}",
            unpaired(self.z)
        ));
        ui.label("Uma caixa = um orbital espacial. Pauli: no máximo duas setas opostas por caixa. Hund: primeiro ocupe caixas equivalentes com spins paralelos.");
        ui.collapsing("Como ler a notação espectroscópica",|ui|{
            ui.label("Configuração 2p²: 2 é n, p significa l=1 e o expoente conta elétrons. É diferente do símbolo de termo atômico (2S+1)L_J.");
            ui.label("Exemplo de um elétron p: S=1/2, L=1 → termos ²P1/2 e ²P3/2. Para muitos elétrons, é preciso combinar L e S antes de determinar J.");
        });
        ui.small("Estados fundamentais H–Ar pelo esquema Aufbau/Hund. Não calcula termos LS de todos os átomos, energias de Hartree, correlação eletrônica ou exceções de metais de transição.");
    }
}
