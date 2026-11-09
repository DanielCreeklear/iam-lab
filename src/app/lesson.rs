use super::{Lab, theme};
use eframe::egui::{self, RichText};
use iam_lab::course;

impl Lab {
    pub(super) fn lesson(&mut self, ui: &mut egui::Ui, index: usize) {
        let lesson = self.catalog[index];
        ui.label(
            RichText::new(format!("{}  /  TEMA {:02}", lesson.block, index + 1))
                .color(theme::TEAL)
                .small()
                .strong(),
        );
        ui.heading(lesson.title);
        ui.label(
            RichText::new(lesson.question)
                .size(19.0)
                .color(theme::MUTED),
        );
        ui.add_space(8.0);
        ui.horizontal_wrapped(|ui| {
            for (stage, label) in [
                "1 · Entender",
                "2 · Prever e testar",
                "3 · Lembrar",
                "4 · Explicar",
            ]
            .iter()
            .enumerate()
            {
                ui.selectable_value(&mut self.study.stage, stage, *label);
            }
        });
        ui.separator();

        let stage = self.study.stage.min(3);
        self.study.entry(lesson.id).visited[stage] = true;
        match stage {
            0 => self.understand(ui, lesson),
            1 => {
                theme::section_title(ui, "Antes de olhar: qual é a sua previsão?");
                ui.label(lesson.prediction);
                ui.add(
                    egui::TextEdit::multiline(&mut self.study.entry(lesson.id).prediction)
                        .hint_text("Escreva uma frase: o que espera ver e por quê?")
                        .desired_rows(2)
                        .desired_width(f32::INFINITY),
                );
                theme::section_title(ui, "Agora investigue");
                for (i, text) in lesson.actions.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!("{}.", i + 1))
                                .strong()
                                .color(theme::TEAL),
                        );
                        ui.label(*text);
                    });
                }
                ui.add_space(4.0);
                self.experiments.show(lesson.experiment, ui);
                ui.add_space(8.0);
                if ui
                    .button("Terminei de explorar · lembrar agora →")
                    .clicked()
                {
                    self.study.stage = 2;
                }
            }
            2 => self.recall(ui, lesson),
            _ => self.explain(ui, lesson),
        }

        ui.add_space(18.0);
        ui.separator();
        ui.horizontal_wrapped(|ui| {
            if index > 0 && ui.button("← Tema anterior").clicked() {
                self.open(index - 1);
            }
            if ui.button("Visão geral").clicked() {
                self.selected = None;
            }
            if index + 1 < self.catalog.len() && ui.button("Próximo tema →").clicked() {
                self.open(index + 1);
            }
        });
    }

    fn understand(&mut self, ui: &mut egui::Ui, lesson: &course::Lesson) {
        theme::callout(ui, "Objetivo de hoje", lesson.goal);
        ui.small(lesson.prerequisite);
        super::glossary::show(ui);

        let progress = self.study.entry(lesson.id);
        progress.concept_step = progress.concept_step.min(2);
        let step = progress.concept_step;
        theme::section_title(ui, &format!("Uma ideia de cada vez · {} de 3", step + 1));
        ui.label(lesson.explanation[step]);
        ui.add_space(10.0);
        ui.add(
            egui::ProgressBar::new((step + 1) as f32 / 3.0)
                .desired_width(120.0)
                .show_percentage(),
        );
        ui.horizontal(|ui| {
            if step > 0 && ui.button("← Ideia anterior").clicked() {
                self.study.entry(lesson.id).concept_step -= 1;
            }
            if step < 2 {
                if ui.button("Próxima ideia →").clicked() {
                    self.study.entry(lesson.id).concept_step += 1;
                }
            } else if ui.button("Fazer uma previsão →").clicked() {
                self.study.stage = 1;
            }
        });

        ui.collapsing("Ver a relação matemática", |ui| {
            ui.monospace(lesson.formula);
            ui.label(lesson.symbols);
        });
        ui.collapsing("Acompanhar um exemplo resolvido", |ui| {
            for (i, text) in lesson.worked.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(format!("Passo {}", i + 1))
                            .strong()
                            .color(theme::TEAL),
                    );
                    ui.label(*text);
                });
            }
        });
        ui.collapsing("Uma confusão comum", |ui| ui.label(lesson.misconception));
    }

    fn recall(&mut self, ui: &mut egui::Ui, lesson: &course::Lesson) {
        theme::section_title(ui, "Tente lembrar antes de conferir");
        ui.label(RichText::new(lesson.quiz.prompt).strong());
        ui.add(
            egui::TextEdit::multiline(&mut self.study.entry(lesson.id).recall)
                .hint_text("Explique sua ideia em uma frase. Não precisa usar a fórmula.")
                .desired_rows(2)
                .desired_width(f32::INFINITY),
        );
        ui.add_space(6.0);
        for (answer, text) in lesson.quiz.choices.iter().enumerate() {
            if ui
                .radio(self.study.entry(lesson.id).answer == Some(answer), *text)
                .clicked()
            {
                self.study.entry(lesson.id).select(answer);
            }
        }
        let progress = self.study.entry(lesson.id);
        if ui
            .add_enabled(
                progress.answer.is_some() && !progress.checked,
                egui::Button::new("Conferir e receber uma dica"),
            )
            .clicked()
        {
            progress.check(lesson.quiz.correct, course::progress::now_unix());
        }
        if progress.checked {
            let answer = progress.answer.unwrap_or(0).min(2);
            let correct = answer == lesson.quiz.correct;
            ui.add_space(8.0);
            theme::callout(
                ui,
                if correct {
                    "Correto · veja o raciocínio"
                } else {
                    "Vamos revisar o raciocínio"
                },
                lesson.quiz.feedback[answer],
            );
            if correct {
                ui.small(
                    "Esta pergunta volta depois de um intervalo para ajudar a manter a lembrança.",
                );
            } else {
                ui.small("Leia a dica, troque sua resposta e tente mais uma vez.");
            }
        }
        ui.add_space(10.0);
        theme::callout(ui, "Ideia para guardar", lesson.takeaway);
        ui.collapsing("Ver de novo o exemplo", |ui| {
            for text in lesson.worked {
                ui.label(text);
            }
        });
        if ui.button("Escrever o que aprendi →").clicked() {
            self.study.stage = 3;
        }
    }

    fn explain(&mut self, ui: &mut egui::Ui, lesson: &course::Lesson) {
        theme::section_title(ui, "Agora, no papel ou no caderno");
        theme::callout(ui, "Prática curta", lesson.paper_practice);
        if !self.study.entry(lesson.id).prediction.is_empty() {
            ui.label(
                RichText::new("Sua previsão")
                    .small()
                    .strong()
                    .color(theme::TEAL),
            );
            ui.label(&self.study.entry(lesson.id).prediction);
        }
        ui.add_space(8.0);
        ui.add(
            egui::TextEdit::multiline(&mut self.study.entry(lesson.id).notes)
                .hint_text("Complete: Eu pensava que… Agora entendo que… Ainda quero descobrir…")
                .desired_rows(8)
                .desired_width(f32::INFINITY),
        );
        ui.small("O caderno e o progresso são salvos neste computador.");
    }
}
