use super::interactions::lennard_jones;

/// Amostragem de Metropolis NVT em 2D. Passos não representam tempo físico.
pub struct Liquid {
    pub positions: Vec<[f64; 2]>,
    pub side: f64,
    rng: u64,
    pub attempted: u64,
    pub accepted: u64,
}

impl Liquid {
    pub fn new(density: f64) -> Self {
        let side = (36.0 / density).sqrt();
        let positions = (0..36)
            .map(|i| {
                [
                    ((i % 6) as f64 + 0.5) * side / 6.0,
                    ((i / 6) as f64 + 0.5) * side / 6.0,
                ]
            })
            .collect();
        Self {
            positions,
            side,
            rng: 42,
            attempted: 0,
            accepted: 0,
        }
    }
    fn random(&mut self) -> f64 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 7;
        self.rng ^= self.rng << 17;
        (self.rng >> 11) as f64 / ((1u64 << 53) as f64)
    }
    fn pair_energy(&self, a: [f64; 2], b: [f64; 2]) -> f64 {
        let mut delta = [a[0] - b[0], a[1] - b[1]];
        for d in &mut delta {
            *d -= self.side * (*d / self.side).round();
        }
        let r = delta[0].hypot(delta[1]);
        // Potencial truncado e deslocado; cutoff abaixo de metade da caixa.
        if r < 2.5 {
            lennard_jones(r.max(1e-8)) - lennard_jones(2.5)
        } else {
            0.0
        }
    }
    fn particle_energy(&self, index: usize, position: [f64; 2]) -> f64 {
        self.positions
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != index)
            .map(|(_, p)| self.pair_energy(position, *p))
            .sum()
    }
    pub fn energy_per_particle(&self) -> f64 {
        self.positions
            .iter()
            .enumerate()
            .map(|(i, p)| self.particle_energy(i, *p))
            .sum::<f64>()
            / (2.0 * self.positions.len() as f64)
    }
    pub fn sweep(&mut self, temperature: f64) {
        for index in 0..self.positions.len() {
            let old = self.positions[index];
            let trial = [
                (old[0] + 0.3 * (self.random() - 0.5)).rem_euclid(self.side),
                (old[1] + 0.3 * (self.random() - 0.5)).rem_euclid(self.side),
            ];
            let delta = self.particle_energy(index, trial) - self.particle_energy(index, old);
            self.attempted += 1;
            if delta <= 0.0 || self.random() < (-delta / temperature).exp() {
                self.positions[index] = trial;
                self.accepted += 1;
            }
        }
    }
}
