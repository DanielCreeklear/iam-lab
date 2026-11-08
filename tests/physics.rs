use iam_lab::physics::{bonding::*, interactions::*, liquid::Liquid, quantum::*, solid::*};

fn integrate(f: impl Fn(f64) -> f64, end: f64) -> f64 {
    let count = 20_000;
    let step = end / count as f64;
    (0..count).map(|i| f((i as f64 + 0.5) * step) * step).sum()
}

#[test]
fn wavefunctions_are_normalized_and_orthogonal() {
    for n in 1..=6 {
        assert!((integrate(|x| well_wave(n, x, 2.0).powi(2), 2.0) - 1.0).abs() < 1e-9);
    }
    assert!(integrate(|x| well_wave(1, x, 2.0) * well_wave(2, x, 2.0), 2.0).abs() < 1e-9);
    for z in [1.0, 2.0, 5.0] {
        assert!((integrate(|r| radial_1s(r, z), 20.0) - 1.0).abs() < 1e-8);
    }
}

#[test]
fn confinement_and_hydrogen_scalings() {
    assert!((well_energy(2, 1.0) / well_energy(1, 1.0) - 4.0).abs() < 1e-12);
    assert!((well_energy(1, 2.0) / well_energy(1, 1.0) - 0.25).abs() < 1e-12);
    assert!((hydrogen_energy(2, 1.0) + 3.40142325).abs() < 1e-8);
}

#[test]
fn bonding_symmetries_and_equilibrium() {
    assert!((morse(1.4, 4.5, 1.5, 1.4) + 4.5).abs() < 1e-12);
    assert!(morse(1.3, 4.5, 1.5, 1.4) > -4.5);
    assert!(morse(1.5, 4.5, 1.5, 1.4) > -4.5);
    for r in [0.3, 1.4, 5.0] {
        assert!(molecular_wave(0.0, r, true).abs() < 1e-12);
        assert!((molecular_wave(0.7, r, false) - molecular_wave(-0.7, r, false)).abs() < 1e-12);
        assert!(overlap(r) > 0.0 && overlap(r) < 1.0);
    }
}

#[test]
fn pair_potentials_have_expected_limits() {
    assert!((lennard_jones(2.0_f64.powf(1.0 / 6.0)) + 1.0).abs() < 1e-12);
    assert_eq!(lennard_jones(1.0), 0.0);
    assert!(dipole_energy(1.0, 0.0, 0.0) < 0.0);
    assert!(
        dipole_energy(
            1.0,
            std::f64::consts::FRAC_PI_2,
            std::f64::consts::FRAC_PI_2
        ) > 0.0
    );
}

#[test]
fn periodic_liquid_is_finite_and_reproducible() {
    let mut a = Liquid::new(0.85);
    let mut b = Liquid::new(0.85);
    for _ in 0..100 {
        a.sweep(0.1);
        b.sweep(0.1);
    }
    assert_eq!(a.positions, b.positions);
    assert_eq!(a.attempted, 3600);
    assert!(a.accepted > 0 && a.accepted <= a.attempted);
    assert!(a.energy_per_particle().is_finite());
    assert!(
        a.positions
            .iter()
            .flatten()
            .all(|&x| x >= 0.0 && x < a.side)
    );
}

#[test]
fn crystal_counts_and_electronic_occupations() {
    for lattice in [
        Lattice::Simple,
        Lattice::BodyCentered,
        Lattice::FaceCentered,
    ] {
        assert_eq!(
            lattice_points(lattice, 3).len(),
            27 * lattice.properties().0 as usize
        );
    }
    let (v, c) = bands(0.0, 1.1, 0.5);
    assert!((c - v - 1.1).abs() < 1e-12);
    assert_eq!(fermi(0.0, 0.0, 300.0), 0.5);
    assert!(fermi(-1.0, 0.0, 300.0) > 0.999);
    assert!(fermi(1.0, 0.0, 300.0) < 0.001);
}
