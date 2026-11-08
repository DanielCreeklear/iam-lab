mod bonding;
mod interactions;
mod liquid;
mod quantum;
mod solid;

use eframe::egui;
use egui_plot::{Legend, Line, Plot};

#[derive(Default)]
pub struct Experiments {
    quantum: quantum::Quantum,
    bonding: bonding::Bonding,
    interactions: interactions::Interactions,
    liquid: liquid::LiquidView,
    solid: solid::Solid,
}

impl Experiments {
    pub fn show(&mut self, topic: usize, ui: &mut egui::Ui) {
        match topic {
            0 => self.quantum.show(ui),
            1 => self.bonding.valence(ui),
            2 => self.bonding.orbitals(ui),
            3 => self.interactions.show(ui),
            4 => self.liquid.show(ui),
            5 => self.solid.crystals(ui),
            _ => self.solid.bands(ui),
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
        .height(250.0)
        .legend(Legend::default())
        .x_axis_label(x)
        .y_axis_label(y)
        .show(ui, |plot| {
            for (name, points) in lines {
                plot.line(Line::new(name, points));
            }
        });
}
