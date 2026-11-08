/// Lennard-Jones em unidades reduzidas sigma=epsilon=1.
pub fn lennard_jones(r: f64) -> f64 {
    let sixth = r.powi(-6);
    4.0 * (sixth * sixth - sixth)
}

/// Dipolos no plano, vetor entre centros paralelo a x.
/// Energia em unidades de p1*p2/(4*pi*epsilon0*sigma³).
pub fn dipole_energy(r: f64, angle_a: f64, angle_b: f64) -> f64 {
    ((angle_a - angle_b).cos() - 3.0 * angle_a.cos() * angle_b.cos()) / r.powi(3)
}
