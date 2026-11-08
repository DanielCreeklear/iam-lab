/// Sobreposição exata entre dois orbitais 1s de hidrogênio (R/a0).
pub fn overlap(distance: f64) -> f64 {
    (-distance).exp() * (1.0 + distance + distance * distance / 3.0)
}

/// Corte no eixo internuclear de orbitais LCAO normalizados em 3D.
/// O gráfico do corte NÃO é uma distribuição de probabilidade em 1D.
pub fn molecular_wave(x: f64, distance: f64, antibonding: bool) -> f64 {
    let a = (-(x + distance / 2.0).abs()).exp() / std::f64::consts::PI.sqrt();
    let b = (-(x - distance / 2.0).abs()).exp() / std::f64::consts::PI.sqrt();
    let sign = if antibonding { -1.0 } else { 1.0 };
    (a + sign * b) / (2.0 * (1.0 + sign * overlap(distance))).sqrt()
}

/// Potencial fenomenológico de Morse: não é solução ab initio de ligação de valência.
pub fn morse(r: f64, depth: f64, width: f64, equilibrium: f64) -> f64 {
    depth * ((1.0 - (-width * (r - equilibrium)).exp()).powi(2) - 1.0)
}
