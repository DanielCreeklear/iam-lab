//! Shell da aplicação: navegação e composição. Conteúdo mora em course/.
mod glossary;
mod home;
mod lesson;
mod theme;

use crate::views::Experiments;
use eframe::egui;
use iam_lab::course::{
    self, Lesson,
    progress::{LegacyStudy, Study},
};

pub struct Lab {
    study: Study,
    catalog: Vec<&'static Lesson>,
    selected: Option<usize>,
    experiments: Experiments,
    search: String,
}
impl Lab {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        theme::apply(&cc.egui_ctx);
        let mut study: Study = cc
            .storage
            .and_then(|s| eframe::get_value(s, "study-v2"))
            .unwrap_or_else(|| {
                let old: LegacyStudy = cc
                    .storage
                    .and_then(|s| eframe::get_value(s, "study"))
                    .unwrap_or_default();
                Study::migrate(old)
            });
        study.stage = study.stage.min(3);
        Self {
            study,
            catalog: course::lessons(),
            selected: None,
            experiments: Experiments::default(),
            search: String::new(),
        }
    }
    fn open(&mut self, index: usize) {
        self.selected = Some(index);
        self.study.last_lesson = self.catalog[index].id.into();
        self.study.stage = 0;
    }
    fn completed(&self) -> usize {
        self.catalog
            .iter()
            .filter(|l| self.study.passed(l.id))
            .count()
    }
    fn resume(&mut self) {
        let index = self
            .catalog
            .iter()
            .position(|l| l.id == self.study.last_lesson)
            .unwrap_or(0);
        self.selected = Some(index);
    }
    fn sidebar(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("course-navigation")
            .exact_width(270.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.add_space(12.0);
                ui.label(
                    egui::RichText::new("IAM / LAB")
                        .size(25.0)
                        .strong()
                        .color(theme::NAVY),
                );
                ui.label("Michel Mendoza · UFABC 2026.3");
                ui.add_space(16.0);
                if ui
                    .add_sized(
                        [ui.available_width(), 36.0],
                        egui::Button::new("Meu percurso").selected(self.selected.is_none()),
                    )
                    .clicked()
                {
                    self.selected = None;
                }
                ui.add_space(8.0);
                ui.add(
                    egui::TextEdit::singleline(&mut self.search)
                        .hint_text("Buscar um assunto...")
                        .desired_width(f32::INFINITY),
                );
                ui.add_space(10.0);
                egui::ScrollArea::vertical()
                    .id_salt("lesson-list")
                    .max_height((ui.available_height() - 120.0).max(160.0))
                    .show(ui, |ui| {
                        let mut block = "";
                        for index in 0..self.catalog.len() {
                            let item = self.catalog[index];
                            if !self.search.is_empty()
                                && !format!("{} {} {}", item.title, item.goal, item.id)
                                    .to_lowercase()
                                    .contains(&self.search.to_lowercase())
                            {
                                continue;
                            }
                            if block != item.block {
                                ui.add_space(12.0);
                                ui.label(
                                    egui::RichText::new(item.block)
                                        .small()
                                        .strong()
                                        .color(theme::MUTED),
                                );
                                block = item.block;
                            }
                            let status = if self.study.passed(item.id) {
                                "✓"
                            } else {
                                ""
                            };
                            let label = format!("{:02}  {}  {}", index + 1, item.title, status);
                            if ui
                                .add_sized(
                                    [ui.available_width(), 44.0],
                                    egui::Button::new(label)
                                        .wrap()
                                        .selected(self.selected == Some(index)),
                                )
                                .clicked()
                            {
                                self.open(index);
                            }
                            ui.add_space(4.0);
                        }
                    });
                ui.separator();
                ui.label(format!(
                    "{} / {} checagens concluídas",
                    self.completed(),
                    self.catalog.len()
                ));
                ui.add(
                    egui::ProgressBar::new(self.completed() as f32 / self.catalog.len() as f32)
                        .fill(theme::TEAL),
                );
                ui.small("Progresso indica prática, não domínio garantido.");
                ui.hyperlink_to("Abrir site do professor", course::PROFESSOR_URL);
            });
    }
}
impl eframe::App for Lab {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "study-v2", &self.study);
    }
    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(15)
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.sidebar(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .id_salt(("main", self.selected, self.study.stage))
                .show(ui, |ui| {
                    ui.set_max_width(1100.0);
                    ui.add_space(14.0);
                    match self.selected {
                        Some(index) => self.lesson(ui, index),
                        None => self.home(ui),
                    }
                    ui.add_space(24.0);
                });
        });
    }
}
