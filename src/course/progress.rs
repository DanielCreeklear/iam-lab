//! Estado de estudo versionado por identificadores estáveis, não por posição no menu.
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct LessonProgress {
    pub prediction: String,
    pub notes: String,
    pub visited: [bool; 4],
    pub answer: Option<usize>,
    pub checked: bool,
    pub passed: bool,
    pub attempts: u32,
}
impl LessonProgress {
    pub fn check(&mut self, correct: usize) {
        if self.answer.is_none() {
            return;
        }
        self.attempts += 1;
        self.checked = true;
        self.passed = self.answer == Some(correct);
    }
    pub fn select(&mut self, answer: usize) {
        self.answer = Some(answer);
        self.checked = false;
    }
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Study {
    pub last_lesson: String,
    pub stage: usize,
    pub lessons: BTreeMap<String, LessonProgress>,
    /// Preserva integralmente as anotações v0.1; sem atribuir domínio ao novo conteúdo.
    pub legacy_notes: Vec<(String, String)>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct LegacyStudy {
    pub selected: usize,
    pub completed: [bool; 7],
    pub notes: [String; 7],
}

impl Study {
    pub fn migrate(old: LegacyStudy) -> Self {
        let names = [
            "Fundamentos quânticos",
            "Ligação de valência",
            "Orbitais moleculares",
            "Interações elétricas",
            "Líquidos",
            "Cristais",
            "Bandas",
        ];
        let legacy_notes = old
            .notes
            .into_iter()
            .enumerate()
            .filter(|(_, s)| !s.is_empty())
            .map(|(i, s)| (names[i].into(), s))
            .collect();
        Self {
            legacy_notes,
            ..Default::default()
        }
    }
    pub fn entry(&mut self, id: &str) -> &mut LessonProgress {
        self.lessons.entry(id.into()).or_default()
    }
    pub fn passed(&self, id: &str) -> bool {
        self.lessons.get(id).is_some_and(|p| p.passed)
    }
}
