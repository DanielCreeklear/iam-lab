//! Orbitais reais de hidrogênio. Coordenadas em a0 e amplitude em a0^(-3/2).
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Orbital {
    #[default]
    S1,
    S2,
    Px2,
    Py2,
    Pz2,
}
impl Orbital {
    pub const ALL: [Self; 5] = [Self::S1, Self::S2, Self::Px2, Self::Py2, Self::Pz2];
    pub fn label(self) -> &'static str {
        match self {
            Self::S1 => "1s",
            Self::S2 => "2s",
            Self::Px2 => "2px",
            Self::Py2 => "2py",
            Self::Pz2 => "2pz",
        }
    }
    pub fn n(self) -> u32 {
        if self == Self::S1 { 1 } else { 2 }
    }
    pub fn wave(self, p: [f64; 3]) -> f64 {
        let r = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
        match self {
            Self::S1 => (-r).exp() / PI.sqrt(),
            Self::S2 => (2.0 - r) * (-r / 2.0).exp() / (4.0 * (2.0 * PI).sqrt()),
            _ => {
                let axis = match self {
                    Self::Px2 => 0,
                    Self::Py2 => 1,
                    _ => 2,
                };
                p[axis] * (-r / 2.0).exp() / (4.0 * (2.0 * PI).sqrt())
            }
        }
    }
    pub fn radial_probability(self, r: f64) -> f64 {
        match self {
            Self::S1 => 4.0 * r * r * (-2.0 * r).exp(),
            Self::S2 => r * r * (2.0 - r).powi(2) * (-r).exp() / 8.0,
            _ => r.powi(4) * (-r).exp() / 24.0,
        }
    }
}

/// Nuvem estática de amostras independentes de |psi|², truncada ao cubo indicado.
/// Rejeição uniforme e semente fixa: os pontos não são trajetórias de elétrons.
pub fn probability_cloud(orbital: Orbital, count: usize) -> Vec<([f64; 3], bool)> {
    let extent = if orbital == Orbital::S1 { 5.0 } else { 14.0 };
    let bound = match orbital {
        Orbital::S1 => 1.0 / PI,
        Orbital::S2 => 1.0 / (8.0 * PI),
        _ => (-2.0_f64).exp() / (8.0 * PI),
    };
    let mut rng = Random::new(2026);
    let mut points = Vec::with_capacity(count);
    // Limite explícito: não há laço ilimitado no thread da interface.
    for _ in 0..count * 3000 {
        let p = std::array::from_fn(|_| (2.0 * rng.sample_unit() - 1.0) * extent);
        let amplitude = orbital.wave(p);
        if rng.sample_unit() * bound <= amplitude * amplitude {
            points.push((p, amplitude >= 0.0));
        }
        if points.len() == count {
            break;
        }
    }
    points
}

pub struct Random(u64);
impl Random {
    pub fn new(seed: u64) -> Self {
        Self(seed.max(1))
    }
    pub fn sample_unit(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
}
