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
        self.study.entry(self.catalog[index].id).concept_step = 0;
    }
    fn start_review(&mut self, index: usize) {
        self.open(index);
        self.study.stage = 2;
        self.study.entry(self.catalog[index].id).start_review();
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
            .frame(egui::Frame::new().fill(theme::RAIL))
            .show(ctx, |ui| {
                ui.add_space(14.0);
                ui.label(
                    egui::RichText::new("IAM / LAB")
                        .size(25.0)
                        .strong()
                        .color(theme::INK),
                );
                ui.label("Quântica, átomos e matéria");
                ui.add_space(12.0);
                if ui
                    .selectable_label(self.selected.is_none(), "Visão geral")
                    .clicked()
                {
                    self.selected = None;
                }
                ui.add_space(8.0);
                ui.add(
                    egui::TextEdit::singleline(&mut self.search)
                        .hint_text("Localizar um tema…")
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
                            let status = self.study.passed(item.id);
                            let label = format!(
                                "{:02}  {}{}",
                                index + 1,
                                item.title,
                                if status { "   ✓" } else { "" }
                            );
                            let selected = self.selected == Some(index);
                            if ui
                                .add_sized(
                                    [ui.available_width(), 42.0],
                                    egui::Button::new(label)
                                        .wrap()
                                        .selected(selected)
                                        .frame(selected),
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
                    "{} de {} conceitos lembrados",
                    self.completed(),
                    self.catalog.len()
                ));
                ui.add(
                    egui::ProgressBar::new(self.completed() as f32 / self.catalog.len() as f32)
                        .fill(theme::TEAL),
                );
                ui.small("O número acompanha respostas corretas; rever ajuda a manter o conteúdo.");
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
                    let available = ui.available_width();
                    let content_width = available.min(1080.0);
                    ui.horizontal(|ui| {
                        ui.add_space(((available - content_width) * 0.5).max(0.0));
                        ui.vertical(|ui| {
                            ui.set_width(content_width);
                            ui.add_space(20.0);
                            match self.selected {
                                Some(index) => self.lesson(ui, index),
                                None => self.home(ui),
                            }
                            ui.add_space(28.0);
                        });
                    });
                });
        });
    }
}
