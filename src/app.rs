use crate::views::Experiments;
use eframe::egui;
use serde::{Deserialize, Serialize};

pub const TOPICS: [&str; 7] = [
    "01 · Fundamentos quânticos",
    "02 · Ligação de valência",
    "03 · Orbitais moleculares",
    "04 · Interações elétricas",
    "05 · Líquidos",
    "06 · Estruturas cristalinas",
    "07 · Bandas e materiais",
];

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
struct Study {
    selected: usize,
    completed: [bool; 7],
    notes: [String; 7],
}

pub struct Lab {
    study: Study,
    experiments: Experiments,
}

impl Lab {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        let mut study: Study = cc
            .storage
            .and_then(|s| eframe::get_value(s, "study"))
            .unwrap_or_default();
        study.selected = study.selected.min(TOPICS.len() - 1);
        Self {
            study,
            experiments: Experiments::default(),
        }
    }
}

impl eframe::App for Lab {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "study", &self.study);
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("navigation").exact_width(240.0).show(ctx, |ui| {
            ui.add_space(12.0);
            ui.heading("IAM Lab");
            ui.label("UFABC · BCK0104-15");
            ui.separator();
            for (index,title) in TOPICS.iter().enumerate() {
                if ui.add_sized(
                    [ui.available_width(), 24.0],
                    egui::Button::new(*title).selected(self.study.selected == index),
                ).clicked() {
                    self.study.selected = index;
                }
                ui.add_space(6.0);
            }
            ui.separator();
            let done = self.study.completed.iter().filter(|&&v| v).count();
            ui.label(format!("Revisados: {done}/7"));
            ui.add(egui::ProgressBar::new(done as f32 / 7.0));
            ui.add_space(16.0);
            ui.label("Roteiro diário\n1. Leia o modelo\n2. Faça uma previsão\n3. Mova um parâmetro\n4. Registre o resultado");
            ui.add_space(16.0);
            ui.hyperlink_to("Ementa e aulas · UFABC", "http://professor.ufabc.edu.br/~pieter.westera/IAM_2503.html");
            ui.small("Projeto independente de estudo. Modelos e aproximações indicados em cada experimento.");
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let selected = self.study.selected;
                ui.heading(TOPICS[selected]);
                ui.add_space(8.0);
                self.experiments.show(selected, ui);
                ui.separator();
                ui.checkbox(&mut self.study.completed[selected], "Revisei este tema");
                ui.label("Minhas previsões e observações (salvas ao fechar)");
                ui.add(
                    egui::TextEdit::multiline(&mut self.study.notes[selected])
                        .desired_rows(3)
                        .desired_width(f32::INFINITY),
                );
            });
        });
    }
}
