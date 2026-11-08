//! Soluções analíticas; comprimento em nm e energia em eV, salvo indicação.
use super::quantum::{well_energy, well_wave};
use std::f64::consts::PI;

/// Superposição normalizada dos estados 1 e 2. `phase` é (E2-E1)t/hbar.
pub fn superposition_density(x: f64, length: f64, weight: f64, phase: f64) -> f64 {
    let a = weight.sqrt() * well_wave(1, x, length);
    let b = (1.0 - weight).sqrt() * well_wave(2, x, length);
    a * a + b * b + 2.0 * a * b * phase.cos()
}

pub fn interval_probability(n: u32, a: f64, b: f64, length: f64) -> f64 {
    let primitive =
        |x: f64| x / length - (2.0 * n as f64 * PI * x / length).sin() / (2.0 * n as f64 * PI);
    primitive(b) - primitive(a)
}

/// Produto Delta x Delta p / hbar no poço [0,L].
pub fn uncertainty_product(n: u32) -> f64 {
    ((n as f64 * PI).powi(2) / 12.0 - 0.5).sqrt()
}

/// Oscilador em coordenada adimensional q=x/sqrt(hbar/(m omega)).
pub fn oscillator_wave(n: u32, q: f64) -> f64 {
    let mut previous = PI.powf(-0.25) * (-q * q / 2.0).exp();
    if n == 0 {
        return previous;
    }
    let mut current = 2.0_f64.sqrt() * q * previous;
    for k in 1..n {
        let next = (2.0 / (k + 1) as f64).sqrt() * q * current
            - (k as f64 / (k + 1) as f64).sqrt() * previous;
        previous = current;
        current = next;
    }
    current
}

pub fn box_energy(numbers: [u32; 3], lengths: [f64; 3]) -> f64 {
    (0..3).map(|i| well_energy(numbers[i], lengths[i])).sum()
}

/// Sem dimensões: u_i=x_i/L_i em [0,1], amplitude normalizada nesse cubo.
pub fn box_wave(numbers: [u32; 3], position: [f64; 3]) -> f64 {
    (0..3)
        .map(|i| 2.0_f64.sqrt() * (numbers[i] as f64 * PI * position[i]).sin())
        .product()
}
