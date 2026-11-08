//! Modelo de um elétron: s=1/2, g_s=2, sem estrutura hiperfina.
pub const BOHR_MAGNETON_EV_T: f64 = 5.788_381_806e-5;
pub const BOHR_MAGNETON_J_T: f64 = 9.274_010_078e-24;
pub const HYDROGEN_MASS_KG: f64 = 1.673_557_5e-27;

pub fn lande(l: u32, j: f64) -> f64 {
    1.0 + (j * (j + 1.0) + 0.75 - l as f64 * (l as f64 + 1.0)) / (2.0 * j * (j + 1.0))
}
/// <L.S>/hbar², s=1/2.
pub fn spin_orbit_factor(l: u32, j: f64) -> f64 {
    (j * (j + 1.0) - l as f64 * (l as f64 + 1.0) - 0.75) / 2.0
}
pub fn zeeman_shift(l: u32, j: f64, mj: f64, field: f64) -> f64 {
    BOHR_MAGNETON_EV_T * lande(l, j) * mj * field
}
/// H(1s), m_s=+1/2: mu_z=-mu_B, Fz=-mu_B*dB/dz.
/// z(x) inclui deriva livre depois de um ímã de comprimento `magnet`.
pub fn stern_gerlach_z(x: f64, speed: f64, gradient: f64, magnet: f64) -> f64 {
    let acceleration = -BOHR_MAGNETON_J_T * gradient / HYDROGEN_MASS_KG;
    let inside = x.min(magnet) / speed;
    let drift = (x - magnet).max(0.0) / speed;
    0.5 * acceleration * inside * inside + acceleration * inside * drift
}
