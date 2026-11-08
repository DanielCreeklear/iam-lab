#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Lattice {
    #[default]
    Simple,
    BodyCentered,
    FaceCentered,
}

impl Lattice {
    pub fn label(self) -> &'static str {
        match self {
            Self::Simple => "Cúbica simples",
            Self::BodyCentered => "Cúbica de corpo centrado",
            Self::FaceCentered => "Cúbica de faces centradas",
        }
    }
    pub fn properties(self) -> (u32, u32, f64) {
        match self {
            Self::Simple => (1, 6, std::f64::consts::PI / 6.0),
            Self::BodyCentered => (2, 8, std::f64::consts::PI * 3.0_f64.sqrt() / 8.0),
            Self::FaceCentered => (4, 12, std::f64::consts::PI / (3.0 * 2.0_f64.sqrt())),
        }
    }
}

/// Células periódicas semiabertas: não duplica sítios das fronteiras.
pub fn lattice_points(lattice: Lattice, cells: u32) -> Vec<[f64; 3]> {
    let basis: &[[f64; 3]] = match lattice {
        Lattice::Simple => &[[0.0, 0.0, 0.0]],
        Lattice::BodyCentered => &[[0.0, 0.0, 0.0], [0.5, 0.5, 0.5]],
        Lattice::FaceCentered => &[
            [0.0, 0.0, 0.0],
            [0.0, 0.5, 0.5],
            [0.5, 0.0, 0.5],
            [0.5, 0.5, 0.0],
        ],
    };
    let mut points = Vec::new();
    for x in 0..cells {
        for y in 0..cells {
            for z in 0..cells {
                for b in basis {
                    points.push([x as f64 + b[0], y as f64 + b[1], z as f64 + b[2]]);
                }
            }
        }
    }
    points
}

/// Duas bandas simétricas didáticas; gap direto mínimo em k=0.
pub fn bands(k: f64, gap: f64, hopping: f64) -> (f64, f64) {
    let dispersion = 2.0 * hopping * (1.0 - k.cos());
    (-gap / 2.0 - dispersion, gap / 2.0 + dispersion)
}

/// Ocupação Fermi-Dirac, energia e potencial químico em eV.
pub fn fermi(energy: f64, chemical_potential: f64, temperature: f64) -> f64 {
    1.0 / (1.0 + ((energy - chemical_potential) / (8.617_333_262e-5 * temperature)).exp())
}
