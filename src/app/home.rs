use super::{Lab, theme};
use eframe::egui::{self, RichText};
use iam_lab::course;
impl Lab {
    pub(super) fn home(&mut self, ui: &mut egui::Ui) {
        ui.label(
            RichText::new("SEU ESPAÇO DE ESTUDO")
                .small()
                .strong()
                .color(theme::TEAL),
        );
        ui.heading("Entenda uma ideia de cada vez.");
        ui.label("Do significado da função de onda à estrutura da matéria. Uma trilha para acompanhar IAM, com explicações, experimentos e feedback.");
        ui.add_space(12.0);
        theme::card(ui, |ui| {
            let index = self
                .catalog
                .iter()
                .position(|l| l.id == self.study.last_lesson)
                .unwrap_or(0);
            ui.label(
                RichText::new(if self.study.last_lesson.is_empty() {
                    "COMECE PELO SIGNIFICADO"
                } else {
                    "CONTINUE DE ONDE PAROU"
                })
                .small()
                .color(theme::MUTED),
            );
            ui.heading(self.catalog[index].title);
            ui.label(self.catalog[index].goal);
            ui.label(format!(
                "Aula {:02} · cerca de {} min · sem limite de tempo",
                index + 1,
                self.catalog[index].minutes
            ));
            if ui
                .button(
                    RichText::new("Continuar estudando  →")
                        .strong()
                        .color(theme::TEAL),
                )
                .clicked()
            {
                self.resume();
            }
        });
        ui.add_space(12.0);
        ui.columns(3, |columns| {
            for (column, (title, body)) in columns.iter_mut().zip([
                (
                    "1. Entender",
                    "Uma ideia central, símbolos explicados e um exemplo resolvido.",
                ),
                (
                    "2. Explorar",
                    "Faça uma previsão. Mude parâmetros e compare com o que imaginou.",
                ),
                (
                    "3. Conferir",
                    "Responda, leia o feedback e escreva sua própria explicação.",
                ),
            ]) {
                theme::card(column, |ui| {
                    ui.strong(title);
                    ui.label(body);
                });
            }
        });
        ui.add_space(12.0);
        theme::callout(
            ui,
            "Material da sua turma · 2026.3",
            "A trilha segue os temas do cronograma e da lista de Michel Mendoza. A numeração abaixo é do aplicativo, não a numeração dos PDFs de aula.",
        );
        ui.horizontal_wrapped(|ui| {
            ui.hyperlink_to("Cronograma oficial", course::SCHEDULE_URL);
            ui.hyperlink_to("Lista de problemas", course::PROBLEMS_URL);
            ui.hyperlink_to("Notas de aula no site", course::PROFESSOR_URL);
        });
        ui.add_space(12.0);
        ui.heading("O caminho completo");
        super::glossary::show(ui);
        let mut previous = "";
        for index in 0..self.catalog.len() {
            let item = self.catalog[index];
            if previous != item.block {
                ui.add_space(12.0);
                ui.strong(item.block);
                previous = item.block;
            }
            theme::card(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.strong(format!("{:02}  {}", index + 1, item.title));
                    if self.study.passed(item.id) {
                        ui.colored_label(theme::TEAL, "Checagem concluída");
                    }
                });
                ui.label(item.question);
                ui.horizontal(|ui| {
                    ui.small(format!("{} min sugeridos", item.minutes));
                    if ui.button("Abrir aula →").clicked() {
                        self.open(index);
                    }
                });
            });
        }
        if !self.study.legacy_notes.is_empty() {
            ui.add_space(16.0);
            ui.collapsing("Suas anotações da versão anterior (preservadas)", |ui| {
                for (title, note) in &mut self.study.legacy_notes {
                    ui.strong(title.as_str());
                    ui.add(egui::TextEdit::multiline(note).desired_width(f32::INFINITY));
                }
            });
        }
    }
}
