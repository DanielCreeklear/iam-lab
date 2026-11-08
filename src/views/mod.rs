mod angular;
mod bonding;
mod electrons;
mod interactions;
mod liquid;
mod oscillator;
mod solid;
mod spatial;
mod spin;
mod superposition;
mod well;

use eframe::egui;
use egui_plot::{Legend, Line, Plot};
use iam_lab::course::Experiment;

#[derive(Default)]
pub struct Experiments {
    well: well::Well,
    superposition: superposition::Superposition,
    oscillator: oscillator::Oscillator,
    spatial: spatial::Spatial,
    angular: angular::Angular,
    spin: spin::Spin,
    electrons: electrons::Electrons,
    bonding: bonding::Bonding,
    interactions: interactions::Interactions,
    liquid: liquid::LiquidView,
    solid: solid::Solid,
    molecule_tab: usize,
    solid_tab: usize,
}

impl Experiments {
    pub fn show(&mut self, topic: Experiment, ui: &mut egui::Ui) {
        match topic {
            Experiment::Eigenstates => self.well.eigenstates(ui),
            Experiment::Well => self.well.probability(ui),
            Experiment::Superposition => self.superposition.show(ui),
            Experiment::Oscillator => self.oscillator.show(ui),
            Experiment::Box3d => self.spatial.box3d(ui),
            Experiment::Hydrogen => self.spatial.hydrogen(ui),
            Experiment::Angular => self.angular.show(ui),
            Experiment::Spin => self.spin.stern_gerlach(ui),
            Experiment::Zeeman => self.spin.zeeman(ui),
            Experiment::Electrons => self.electrons.show(ui),
            Experiment::Molecules => {
                ui.horizontal_wrapped(|ui| {
                    for (i, name) in [
                        "Orbitais moleculares",
                        "Ligação de valência",
                        "Dipolos (complemento)",
                        "Fluido (complemento)",
                    ]
                    .iter()
                    .enumerate()
                    {
                        ui.selectable_value(&mut self.molecule_tab, i, *name);
                    }
                });
                match self.molecule_tab {
                    0 => self.bonding.orbitals(ui),
                    1 => self.bonding.valence(ui),
                    2 => self.interactions.show(ui),
                    _ => self.liquid.show(ui),
                }
            }
            Experiment::Solids => {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.solid_tab, 0, "Cristais 3D");
                    ui.selectable_value(&mut self.solid_tab, 1, "Bandas e ocupação");
                });
                if self.solid_tab == 0 {
                    self.solid.crystals(ui)
                } else {
                    self.solid.bands(ui)
                }
            }
        }
    }
}

pub fn curve(start: f64, end: f64, f: impl Fn(f64) -> f64) -> Vec<[f64; 2]> {
    (0..=400)
        .map(|i| {
            let x = start + (end - start) * i as f64 / 400.0;
            [x, f(x)]
        })
        .collect()
}

pub fn graph(ui: &mut egui::Ui, id: &str, x: &str, y: &str, lines: Vec<(&str, Vec<[f64; 2]>)>) {
    Plot::new(id)
        .height(240.0)
        .legend(Legend::default())
        .x_axis_label(x)
        .y_axis_label(y)
        .show(ui, |plot| {
            for (name, points) in lines {
                plot.line(Line::new(name, points));
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_lesson_experiment_builds_a_frame() {
        let context = egui::Context::default();
        let mut experiments = Experiments::default();
        for lesson in iam_lab::course::lessons() {
            let output = context.run(egui::RawInput::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    experiments.show(lesson.experiment, ui);
                });
            });
            assert!(!output.shapes.is_empty(), "{}", lesson.id);
        }
    }
}
