//! Conteúdo pedagógico puro: não depende da janela, do renderer ou do armazenamento.
mod atoms;
mod foundations;
mod matter;
pub mod progress;

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
    pub paper_practice: &'static str,
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
