use super::{curve, graph};
use crate::render::scene::{Camera, NEGATIVE, POSITIVE, Point};
use eframe::egui;
use iam_lab::physics::{
    confinement::*,
    orbitals::{Orbital, Random, probability_cloud},
    quantum::hydrogen_energy,
};

pub struct Spatial {
    orbital: Orbital,
    cached_orbital: Option<Orbital>,
    cloud: Vec<Point>,
    orbital_camera: Camera,
    slice: bool,
    numbers: [u32; 3],
    lengths: [f64; 3],
    box_cache: Option<[u32; 3]>,
    box_samples: Vec<([f64; 3], bool)>,
    box_camera: Camera,
}
impl Default for Spatial {
    fn default() -> Self {
        Self {
            orbital: Orbital::S1,
            cached_orbital: None,
            cloud: Vec::new(),
            orbital_camera: Camera::default(),
            slice: false,
            numbers: [1, 1, 1],
            lengths: [1.0; 3],
            box_cache: None,
            box_samples: Vec::new(),
            box_camera: Camera::default(),
        }
    }
}
impl Spatial {
    pub fn hydrogen(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            for orbital in Orbital::ALL {
                ui.selectable_value(&mut self.orbital, orbital, orbital.label());
            }
            ui.checkbox(&mut self.slice, "Corte |y|<0,6 a0");
        });
        if self.cached_orbital != Some(self.orbital) {
            self.cloud = probability_cloud(self.orbital, 1800)
                .into_iter()
                .map(|(position, positive)| Point { position, positive })
                .collect();
            self.cached_orbital = Some(self.orbital);
            self.orbital_camera = Camera::default();
        }
        let extent = if self.orbital == Orbital::S1 {
            5.0
        } else {
            14.0
        };
        let sliced: Vec<_> = self
            .cloud
            .iter()
            .filter(|p| !self.slice || p.position[1].abs() < 0.6)
            .map(|p| Point {
                position: p.position,
                positive: p.positive,
            })
            .collect();
        self.orbital_camera
            .show(ui, self.orbital.label(), &sliced, extent, "a0", None);
        ui.horizontal_wrapped(|ui| {
            ui.colored_label(POSITIVE, "● ψ positivo");
            ui.colored_label(NEGATIVE, "● ψ negativo");
            ui.label("Densidade de pontos ∝ |ψ|². Cor não é carga.");
        });
        ui.label(format!(
            "n={} · E={:.3} eV · {} amostras no volume (antes do corte)",
            self.orbital.n(),
            hydrogen_energy(self.orbital.n(), 1.0),
            self.cloud.len()
        ));
        graph(
            ui,
            "hydrogen-radial",
            "r (a0)",
            "P(r) (a0^−1)",
            vec![(
                "Probabilidade radial",
                curve(0.0, extent, |r| self.orbital.radial_probability(r)),
            )],
        );
        ui.small("Hidrogênio não relativístico, Z=1, sem spin. Nuvem truncada no cubo ±5 a0 (1s) ou ±14 a0 (n=2); distribuição visual condicionada a esse volume. Corte é uma fatia de espessura finita, não uma isosuperfície.");
    }
    pub fn box3d(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            for state in [[1, 1, 1], [2, 1, 1], [1, 2, 1], [1, 1, 2]] {
                if ui
                    .button(format!("({},{},{})", state[0], state[1], state[2]))
                    .clicked()
                {
                    self.numbers = state;
                }
            }
            if ui.button("Cubo de 1 nm").clicked() {
                self.lengths = [1.0; 3];
            }
        });
        for (axis, label) in ["x", "y", "z"].iter().enumerate() {
            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut self.numbers[axis], 1..=4).text(format!("n{label}")));
                ui.add(
                    egui::Slider::new(&mut self.lengths[axis], 0.5..=2.0)
                        .text(format!("L{label} (nm)")),
                );
            });
        }
        if self.box_cache != Some(self.numbers) {
            let mut rng = Random::new(1234);
            self.box_samples.clear();
            for _ in 0..40_000 {
                let p = std::array::from_fn(|_| rng.sample_unit());
                let wave = box_wave(self.numbers, p);
                if rng.sample_unit() * 8.0 <= wave * wave {
                    self.box_samples.push((p, wave >= 0.0));
                }
                if self.box_samples.len() == 1800 {
                    break;
                }
            }
            self.box_cache = Some(self.numbers);
        }
        let points: Vec<_> = self
            .box_samples
            .iter()
            .map(|(p, positive)| Point {
                position: std::array::from_fn(|i| (p[i] - 0.5) * self.lengths[i]),
                positive: *positive,
            })
            .collect();
        self.box_camera
            .show(ui, "Caixa 3D", &points, 1.5, "nm", Some(self.lengths));
        ui.strong(format!(
            "Energia total: {:.4} eV",
            box_energy(self.numbers, self.lengths)
        ));
        ui.label(format!(
            "E(2,1,1)={:.4}  ·  E(1,2,1)={:.4}  ·  E(1,1,2)={:.4} eV",
            box_energy([2, 1, 1], self.lengths),
            box_energy([1, 2, 1], self.lengths),
            box_energy([1, 1, 2], self.lengths)
        ));
        ui.small("Origem visual no centro da caixa; equações usam [0,L] em cada eixo. Paredes infinitas. Azul/laranja indicam sinal de ψ, não cargas. Nuvem de probabilidades, não trajetórias.");
    }
}
