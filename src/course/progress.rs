//! Estado de estudo versionado por identificadores estáveis, não por posição no menu.
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct LessonProgress {
    pub prediction: String,
    pub recall: String,
    pub notes: String,
    pub visited: [bool; 4],
    pub concept_step: usize,
    pub answer: Option<usize>,
    pub checked: bool,
    pub passed: bool,
    pub attempts: u32,
    pub review_streak: u8,
    pub next_review_at: Option<u64>,
}
impl LessonProgress {
    pub fn check(&mut self, correct: usize, now: u64) {
        if self.answer.is_none() {
            return;
        }
        self.attempts += 1;
        self.checked = true;
        if self.answer == Some(correct) {
            self.passed = true;
            let delays_days = [1_u64, 3, 7, 14, 30];
            let delay = delays_days[self.review_streak.min(4) as usize];
            self.next_review_at = Some(now.saturating_add(delay * 86_400));
            self.review_streak = self.review_streak.saturating_add(1).min(5);
        } else {
            self.review_streak = 0;
            self.next_review_at = self.passed.then_some(now.saturating_add(86_400));
        }
    }
    pub fn select(&mut self, answer: usize) {
        self.answer = Some(answer);
        self.checked = false;
    }
    pub fn start_review(&mut self) {
        self.answer = None;
        self.checked = false;
        self.recall.clear();
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
    pub fn review_due(&self, id: &str, now: u64) -> bool {
        self.lessons
            .get(id)
            .is_some_and(|p| p.passed && p.next_review_at.is_some_and(|t| t <= now))
    }
    pub fn due_reviews(&self, ids: impl IntoIterator<Item = &'static str>, now: u64) -> usize {
        ids.into_iter()
            .filter(|id| self.review_due(id, now))
            .count()
    }
}

pub fn now_unix() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
