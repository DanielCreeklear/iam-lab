use super::{curve, graph};
use crate::render::scene::{Camera, Point};
use eframe::egui;
use iam_lab::physics::solid::*;

pub struct Solid {
    lattice: Lattice,
    cells: u32,
    camera: Camera,
    gap: f64,
    hopping: f64,
    temperature: f64,
    mu: f64,
}
impl Default for Solid {
    fn default() -> Self {
        Self {
            lattice: Lattice::Simple,
            cells: 3,
            camera: Camera::default(),
            gap: 1.1,
            hopping: 0.5,
            temperature: 300.0,
            mu: 0.0,
        }
    }
}
impl Solid {
    pub fn crystals(&mut self, ui: &mut egui::Ui) {
        ui.label("Compare redes cúbicas em uma projeção de seus sítios tridimensionais.");
        egui::ComboBox::from_label("Estrutura")
            .selected_text(self.lattice.label())
            .show_ui(ui, |ui| {
                for lattice in [
                    Lattice::Simple,
                    Lattice::BodyCentered,
                    Lattice::FaceCentered,
                ] {
                    ui.selectable_value(&mut self.lattice, lattice, lattice.label());
                }
            });
        ui.add(egui::Slider::new(&mut self.cells, 1..=5).text("Células por eixo"));
        let (atoms, coordination, packing) = self.lattice.properties();
        ui.label(format!(
            "Átomos/célula: {atoms} · coordenação: {coordination} · empacotamento: {:.2}%",
            packing * 100.0
        ));
        let center = self.cells as f64 / 2.0;
        let points: Vec<_> = lattice_points(self.lattice, self.cells)
            .iter()
            .map(|p| Point {
                position: p.map(|v| v - center),
                positive: true,
            })
            .collect();
        self.camera.show(
            ui,
            self.lattice.label(),
            &points,
            self.cells as f64,
            "a",
            Some([self.cells as f64; 3]),
        );
        ui.label("Experimente: compare a coordenação da cúbica simples e da CFC. Por que CFC ocupa maior fração do volume?");
        ui.small("Projeção ortográfica de células semiabertas, sem repetir átomos das faces externas. Pontos não têm raio físico; o empacotamento usa esferas rígidas em contato. Coordenação refere-se ao cristal infinito.");
    }
    pub fn bands(&mut self, ui: &mut egui::Ui) {
        ui.label("Modelo didático de duas bandas 1D: dispersão, gap e ocupação eletrônica.");
        ui.add(
            egui::Slider::new(&mut self.gap, -1.0..=5.0).text("Gap (eV); negativo = sobreposição"),
        );
        ui.add(egui::Slider::new(&mut self.hopping, 0.1..=2.0).text("Acoplamento t (eV)"));
        ui.add(egui::Slider::new(&mut self.temperature, 10.0..=1500.0).text("Temperatura (K)"));
        ui.add(egui::Slider::new(&mut self.mu, -3.0..=3.0).text("Potencial químico μ (eV)"));
        ui.monospace("Ec,v = ±[Eg/2 + 2t(1−cos ka)]\nf(E) = 1/[1+exp((E−μ)/(kBT))]");
        let pi = std::f64::consts::PI;
        graph(
            ui,
            "bands",
            "ka",
            "E (eV)",
            vec![
                (
                    "Banda inferior",
                    curve(-pi, pi, |k| bands(k, self.gap, self.hopping).0),
                ),
                (
                    "Banda superior",
                    curve(-pi, pi, |k| bands(k, self.gap, self.hopping).1),
                ),
                ("μ", vec![[-pi, self.mu], [pi, self.mu]]),
            ],
        );
        graph(
            ui,
            "fermi",
            "E (eV)",
            "ocupação f(E)",
            vec![(
                "Fermi-Dirac",
                curve(-5.0, 5.0, |e| fermi(e, self.mu, self.temperature)),
            )],
        );
        ui.label("Experimente: aumente T. O que acontece com a ocupação perto de μ? Compare gap positivo e bandas sobrepostas.");
        ui.small("Duas dispersões parametrizadas, não uma estrutura de bandas de material real. μ é controlado, não calculado por neutralidade. Sem densidade de estados, transporte, dopagem explícita ou limiar universal entre semicondutor e isolante.");
    }
}
