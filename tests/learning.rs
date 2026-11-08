use iam_lab::{
    course::{self, progress::*},
    physics::{confinement::*, electrons::*, orbitals::*, quantum::*, spin::*},
};

fn integrate(f: impl Fn(f64) -> f64, a: f64, b: f64) -> f64 {
    let dx = (b - a) / 20_000.0;
    (0..20_000).map(|i| f(a + (i as f64 + 0.5) * dx) * dx).sum()
}
#[test]
fn superposition_conserves_probability_for_all_phases() {
    for weight in [0.0, 0.2, 0.5, 1.0] {
        for phase in [0.0, 1.0, 3.0, 5.0] {
            assert!(
                (integrate(|x| superposition_density(x, 1.7, weight, phase), 0.0, 1.7) - 1.0).abs()
                    < 1e-9
            );
        }
    }
    assert!((interval_probability(2, 0.0, 0.5, 1.0) - 0.5).abs() < 1e-12);
    assert!(uncertainty_product(1) > 0.5);
}
#[test]
fn oscillator_states_normalized_orthogonal_and_zero_point_nonzero() {
    for n in 0..=6 {
        assert!((integrate(|q| oscillator_wave(n, q).powi(2), -10.0, 10.0) - 1.0).abs() < 1e-9);
    }
    assert!(
        integrate(
            |q| oscillator_wave(0, q) * oscillator_wave(2, q),
            -10.0,
            10.0
        )
        .abs()
            < 1e-9
    );
}
#[test]
fn box_degeneracy_is_broken_by_asymmetry() {
    let e = box_energy([2, 1, 1], [1.0; 3]);
    assert!((e - box_energy([1, 2, 1], [1.0; 3])).abs() < 1e-12);
    assert!(box_energy([2, 1, 1], [2.0, 1.0, 1.0]) < box_energy([1, 2, 1], [2.0, 1.0, 1.0]));
    assert!((box_energy([1, 1, 1], [1.0; 3]) - 3.0 * well_energy(1, 1.0)).abs() < 1e-12);
    assert!(box_wave([2, 1, 1], [0.5, 0.2, 0.3]).abs() < 1e-12);
}
#[test]
fn hydrogen_radial_normalization_and_nodes() {
    for orbital in Orbital::ALL {
        assert!((integrate(|r| orbital.radial_probability(r), 0.0, 60.0) - 1.0).abs() < 1e-8);
    }
    assert_eq!(Orbital::S2.wave([2.0, 0.0, 0.0]), 0.0);
    assert_eq!(Orbital::Pz2.wave([1.0, 1.0, 0.0]), 0.0);
    assert!(Orbital::Pz2.wave([0.0, 0.0, 1.0]) > 0.0);
    assert!(Orbital::Pz2.wave([0.0, 0.0, -1.0]) < 0.0);
}
#[test]
fn cloud_sampling_agrees_with_analytic_radial_moment() {
    let cloud = probability_cloud(Orbital::S1, 3000);
    assert_eq!(cloud.len(), 3000);
    let mean_r = cloud
        .iter()
        .map(|(p, _)| (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt())
        .sum::<f64>()
        / cloud.len() as f64;
    // <r> = 3a0/2. Tolera amostragem finita e cauda truncada da visualização.
    assert!((mean_r - 1.5).abs() < 0.1, "mean radius {mean_r}");
    for orbital in [Orbital::S2, Orbital::Pz2] {
        assert_eq!(probability_cloud(orbital, 200).len(), 200);
    }
}
#[test]
fn angular_momentum_and_weak_field_references() {
    assert!((lande(1, 0.5) - 2.0 / 3.0).abs() < 1e-12);
    assert!((lande(1, 1.5) - 4.0 / 3.0).abs() < 1e-12);
    assert_eq!(spin_orbit_factor(1, 0.5), -1.0);
    assert_eq!(spin_orbit_factor(1, 1.5), 0.5);
    assert_eq!(zeeman_shift(1, 1.5, 0.5, 0.0), 0.0);
    assert!((zeeman_shift(0, 0.5, 0.5, 1.0) - BOHR_MAGNETON_EV_T).abs() < 1e-12);
}
#[test]
fn stern_gerlach_gradient_speed_and_drift() {
    assert_eq!(stern_gerlach_z(0.4, 2000.0, 0.0, 0.2), 0.0);
    let z = stern_gerlach_z(0.4, 2000.0, 100.0, 0.2);
    assert!(z < 0.0);
    assert!((stern_gerlach_z(0.4, 4000.0, 100.0, 0.2) / z - 0.25).abs() < 1e-12);
    // Tempos iguais dentro do ímã e na deriva: z_final=3*z_saida.
    assert!((z / stern_gerlach_z(0.2, 2000.0, 100.0, 0.2) - 3.0).abs() < 1e-12);
}
#[test]
fn aufbau_hund_conserve_electrons_and_pauli() {
    for z in 1..=18 {
        let states: Vec<_> = configuration(z)
            .iter()
            .flat_map(|s| occupancy(s.l, s.electrons))
            .collect();
        assert_eq!(
            states.iter().flatten().filter(|&&filled| filled).count(),
            z as usize
        );
    }
    assert_eq!(unpaired(2), 0);
    assert_eq!(unpaired(4), 0);
    assert_eq!(unpaired(6), 2);
    assert_eq!(unpaired(7), 3);
    assert_eq!(unpaired(8), 2);
}
#[test]
fn quiz_requires_answer_and_feedback_tracks_retries() {
    let mut progress = LessonProgress::default();
    progress.check(1);
    assert_eq!(progress.attempts, 0);
    assert!(!progress.passed);
    progress.select(0);
    progress.check(1);
    assert!(progress.checked);
    assert!(!progress.passed);
    progress.select(1);
    assert!(!progress.checked);
    progress.check(1);
    assert!(progress.passed);
    assert_eq!(progress.attempts, 2);
}
#[test]
fn migration_preserves_old_notes_without_claiming_new_progress() {
    let mut old = LegacyStudy::default();
    old.notes[4] = "Minha observação do fluido".into();
    old.completed[4] = true;
    let new = Study::migrate(old);
    assert_eq!(
        new.legacy_notes[0],
        ("Líquidos".into(), "Minha observação do fluido".into())
    );
    assert!(new.lessons.is_empty());
}
#[test]
fn curriculum_has_unique_keys_and_valid_quizzes() {
    let lessons = course::lessons();
    let mut ids = std::collections::HashSet::new();
    for lesson in lessons {
        assert!(ids.insert(lesson.id));
        assert!(lesson.quiz.correct < 3);
        assert!(lesson.quiz.feedback.iter().all(|s| !s.is_empty()));
        assert!(!lesson.source_section.is_empty());
    }
}
