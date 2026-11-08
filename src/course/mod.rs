//! Conteúdo pedagógico puro: não depende da janela, do renderer ou do armazenamento.
mod atoms;
mod foundations;
mod matter;
pub mod progress;

pub const PROFESSOR_URL: &str = "https://sites.google.com/site/xxmichelmendoza/intera%C3%A7%C3%B5es-at%C3%B4micas-e-moleculares-bck0104-15?authuser=0";
pub const SCHEDULE_URL: &str =
    "https://drive.google.com/file/d/1OynQ46aJRfofeDTO9mGY8H-gkvkMALCe/view";
pub const PROBLEMS_URL: &str =
    "https://drive.google.com/file/d/1Bd__o5EInNnvlc5IL5g1mz-Rd99IJfCg/view";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Experiment {
    Eigenstates,
    Well,
    Superposition,
    Oscillator,
    Box3d,
    Hydrogen,
    Angular,
    Spin,
    Zeeman,
    Electrons,
    Molecules,
    Solids,
}

pub struct Quiz {
    pub prompt: &'static str,
    pub choices: [&'static str; 3],
    pub correct: usize,
    pub feedback: [&'static str; 3],
}
pub struct Lesson {
    pub id: &'static str,
    pub title: &'static str,
    pub block: &'static str,
    pub minutes: u32,
    pub question: &'static str,
    pub goal: &'static str,
    pub prerequisite: &'static str,
    pub explanation: [&'static str; 3],
    pub formula: &'static str,
    pub symbols: &'static str,
    pub worked: [&'static str; 3],
    pub prediction: &'static str,
    pub actions: [&'static str; 3],
    pub takeaway: &'static str,
    pub misconception: &'static str,
    pub source_section: &'static str,
    pub experiment: Experiment,
    pub quiz: Quiz,
}

pub fn lessons() -> Vec<&'static Lesson> {
    foundations::LESSONS
        .iter()
        .chain(atoms::LESSONS.iter())
        .chain(matter::LESSONS.iter())
        .collect()
}
