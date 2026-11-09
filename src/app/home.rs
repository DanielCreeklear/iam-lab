use super::{Lab, theme};
use eframe::egui::{self, RichText};
use iam_lab::course;

impl Lab {
    pub(super) fn home(&mut self, ui: &mut egui::Ui) {
        ui.label(
            RichText::new("LABORATÓRIO DE ESTUDO")
                .small()
                .strong()
                .color(theme::TEAL),
        );
        ui.heading("Interações atômicas e moleculares");
        ui.label(
            "Uma ideia por vez: compreenda, preveja, experimente e explique com suas palavras.",
        );
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.label(format!(
                "{} de {} conceitos conferidos",
                self.completed(),
                self.catalog.len()
            ));
            ui.add(
                egui::ProgressBar::new(self.completed() as f32 / self.catalog.len() as f32)
                    .desired_width(150.0)
                    .fill(theme::TEAL),
            );
        });

        let now = course::progress::now_unix();
        let due = self
            .catalog
            .iter()
            .position(|lesson| self.study.review_due(lesson.id, now));
        let due_count = self
            .catalog
            .iter()
            .filter(|lesson| self.study.review_due(lesson.id, now))
            .count();
        if let Some(index) = due {
            ui.add_space(12.0);
            theme::section_title(ui, "Revisão espaçada");
            ui.label(format!(
                "{} {} para revisar de memória.",
                due_count,
                if due_count == 1 {
                    "ideia pronta"
                } else {
                    "ideias prontas"
                }
            ));
            ui.label("Responda sem reler a explicação; o retorno mostra o que vale revisar.");
            if ui.button("Começar revisão →").clicked() {
                self.start_review(index);
            }
        } else {
            ui.add_space(12.0);
            ui.label(RichText::new("Revisão espaçada").strong().color(theme::INK));
            ui.label("Uma ideia aparece aqui um dia depois de ser lembrada corretamente.");
        }

        ui.add_space(8.0);
        theme::section_title(ui, "Retomar o estudo");
        let index = self
            .catalog
            .iter()
            .position(|l| l.id == self.study.last_lesson)
            .unwrap_or(0);
        let lesson = self.catalog[index];
        ui.label(
            RichText::new(if self.study.last_lesson.is_empty() {
                "COMECE AQUI"
            } else {
                "ÚLTIMA AULA"
            })
            .small()
            .strong()
            .color(theme::TEAL),
        );
        ui.heading(lesson.title);
        ui.label(lesson.question);
        ui.label(format!(
            "Aula {:02} · cerca de {} minutos",
            index + 1,
            lesson.minutes
        ));
        if ui.button("Continuar →").clicked() {
            self.resume();
        }

        ui.add_space(4.0);
        super::theme::section_title(ui, "Seu caminho de estudo");
        super::glossary::show(ui);
        let mut previous = "";
        for index in 0..self.catalog.len() {
            let item = self.catalog[index];
            if previous != item.block {
                ui.add_space(8.0);
                ui.label(
                    RichText::new(item.block)
                        .small()
                        .strong()
                        .color(theme::TEAL),
                );
                previous = item.block;
            }
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!("{:02}", index + 1))
                        .strong()
                        .color(theme::TEAL),
                );
                ui.vertical(|ui| {
                    ui.strong(item.title);
                    ui.label(item.question);
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Abrir →").clicked() {
                        self.open(index);
                    }
                    if self.study.passed(item.id) {
                        ui.small("Lembrado");
                    }
                });
            });
            ui.separator();
        }

        if !self.study.legacy_notes.is_empty() {
            ui.add_space(12.0);
            ui.collapsing("Anotações preservadas", |ui| {
                for (title, note) in &mut self.study.legacy_notes {
                    ui.strong(title.as_str());
                    ui.add(egui::TextEdit::multiline(note).desired_width(f32::INFINITY));
                }
            });
        }
    }
}
