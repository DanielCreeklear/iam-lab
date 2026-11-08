use super::{Lab, theme};
use eframe::egui::{self, RichText};
use iam_lab::course;

impl Lab {
    pub(super) fn lesson(&mut self, ui: &mut egui::Ui, index: usize) {
        let lesson = self.catalog[index];
        ui.label(
            RichText::new(format!("{}  /  AULA {:02}", lesson.block, index + 1))
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
        ui.add_space(10.0);
        ui.horizontal_wrapped(|ui| {
            for (stage, label) in [
                "1 · Entender",
                "2 · Explorar",
                "3 · Conferir",
                "4 · Caderno",
            ]
            .iter()
            .enumerate()
            {
                ui.selectable_value(&mut self.study.stage, stage, *label);
            }
        });
        ui.separator();
        let stage = self.study.stage;
        self.study.entry(lesson.id).visited[stage] = true;
        match self.study.stage {
            0 => {
                theme::callout(ui, "Ao terminar, você deverá conseguir", lesson.goal);
                ui.small(lesson.prerequisite);
                super::glossary::show(ui);
                ui.add_space(8.0);
                theme::card(ui, |ui| {
                    for (i, text) in lesson.explanation.iter().enumerate() {
                        ui.strong(format!("Ideia {}", i + 1));
                        ui.label(*text);
                        ui.add_space(5.0);
                    }
                });
                ui.add_space(10.0);
                theme::card(ui, |ui| {
                    ui.strong("A equação, com significado");
                    ui.monospace(lesson.formula);
                    ui.label(lesson.symbols);
                });
                ui.add_space(10.0);
                ui.collapsing("Exemplo resolvido · veja o raciocínio em 3 passos", |ui| {
                    for (i, text) in lesson.worked.iter().enumerate() {
                        ui.label(format!("{}. {text}", i + 1));
                    }
                });
                theme::callout(ui, "Uma confusão comum", lesson.misconception);
                if ui.button("Agora quero experimentar →").clicked() {
                    self.study.stage = 1;
                }
            }
            1 => {
                theme::callout(ui, "Antes de mexer: faça uma previsão", lesson.prediction);
                let progress = self.study.entry(lesson.id);
                ui.add(
                    egui::TextEdit::multiline(&mut progress.prediction)
                        .hint_text("Acho que... porque...")
                        .desired_rows(2)
                        .desired_width(f32::INFINITY),
                );
                ui.collapsing("Roteiro de exploração (3 passos)", |ui| {
                    for (i, text) in lesson.actions.iter().enumerate() {
                        ui.label(format!("{}. {text}", i + 1));
                    }
                });
                ui.separator();
                self.experiments.show(lesson.experiment, ui);
                ui.add_space(10.0);
                if ui.button("Conferir o que entendi →").clicked() {
                    self.study.stage = 2;
                }
            }
            2 => {
                let progress = self.study.entry(lesson.id);
                theme::card(ui, |ui| {
                    ui.strong("Sem consultar: explique para você mesmo");
                    ui.label(lesson.quiz.prompt);
                    for (answer, text) in lesson.quiz.choices.iter().enumerate() {
                        if ui.radio(progress.answer == Some(answer), *text).clicked() {
                            progress.select(answer);
                        }
                    }
                    if ui
                        .add_enabled(
                            progress.answer.is_some() && !progress.checked,
                            egui::Button::new("Conferir resposta"),
                        )
                        .clicked()
                    {
                        progress.check(lesson.quiz.correct);
                    }
                    if progress.checked {
                        let answer = progress.answer.unwrap_or(0).min(2);
                        let correct = answer == lesson.quiz.correct;
                        ui.add_space(8.0);
                        theme::callout(
                            ui,
                            if correct {
                                "Isso mesmo. Veja por quê:"
                            } else {
                                "Vamos rever o raciocínio:"
                            },
                            lesson.quiz.feedback[answer],
                        );
                        if !correct {
                            ui.label("Você pode escolher outra resposta ou voltar ao experimento. Errar faz parte da investigação.");
                        }
                    }
                });
                ui.add_space(12.0);
                theme::callout(ui, "Guarde esta ideia", lesson.takeaway);
                ui.collapsing("Rever o exemplo resolvido", |ui| {
                    for text in lesson.worked {
                        ui.label(text);
                    }
                });
                if ui.button("Registrar minha explicação →").clicked() {
                    self.study.stage = 3;
                }
            }
            _ => {
                theme::callout(
                    ui,
                    "Explique sem copiar a fórmula",
                    "O que você previa? O que observou? Qual regra física explica a diferença? Qual limite tem este modelo?",
                );
                let progress = self.study.entry(lesson.id);
                if !progress.prediction.is_empty() {
                    ui.label(format!("Sua previsão: {}", progress.prediction));
                }
                ui.add(
                    egui::TextEdit::multiline(&mut progress.notes)
                        .hint_text("Hoje entendi que...\nAinda tenho dúvida sobre...")
                        .desired_rows(9)
                        .desired_width(f32::INFINITY),
                );
                ui.small("Notas e progresso salvos automaticamente a cada 15 s e ao fechar normalmente. Tudo fica neste computador.");
                ui.add_space(10.0);
                ui.strong("Continue na lista do professor");
                ui.label(lesson.source_section);
                ui.hyperlink_to("Abrir lista 2026.3", course::PROBLEMS_URL);
                ui.label("Resolva no papel. As checagens do aplicativo são autorais e não substituem as demonstrações da lista.");
            }
        }
        ui.add_space(18.0);
        ui.separator();
        ui.horizontal_wrapped(|ui| {
            if index > 0 && ui.button("← Aula anterior").clicked() {
                self.open(index - 1);
            }
            if ui.button("Meu percurso").clicked() {
                self.selected = None;
            }
            if index + 1 < self.catalog.len() && ui.button("Próxima aula →").clicked() {
                self.open(index + 1);
            }
        });
    }
}
