use std::f64::consts::PI;

/// Poço infinito: x e L em nm, função de onda em nm^(-1/2).
pub fn well_wave(n: u32, x: f64, length: f64) -> f64 {
    if x < 0.0 || x > length {
        return 0.0;
    }
    (2.0 / length).sqrt() * (n as f64 * PI * x / length).sin()
}

/// Elétron no poço infinito: hbar²/(2 m_e) = 0.0380998 eV nm².
pub fn well_energy(n: u32, length: f64) -> f64 {
    0.038_099_8 * (n as f64 * PI / length).powi(2)
}

/// Densidade radial 1s hidrogenoide, r em raios de Bohr; integral dr = 1.
pub fn radial_1s(r: f64, charge: f64) -> f64 {
    4.0 * charge.powi(3) * r.powi(2) * (-2.0 * charge * r).exp()
}

pub fn hydrogen_energy(n: u32, charge: f64) -> f64 {
    -13.605_693 * charge.powi(2) / (n as f64).powi(2)
}
