use crate::point::Point2D;
use rand::rngs::ThreadRng;
use rand::{Rng, rng};
use rand_distr::{Distribution, Normal};
use statrs::distribution::{Continuous, Normal as StatrsNormal};

const N_PARTICLES: usize = 10000;

#[derive(Clone, Copy, Debug)]
pub struct State {
    pub pos: Point2D<f64>,
    weight: f64,
}

impl State {
    pub fn new() -> Self {
        State {
            pos: Point2D { x: 0.0, y: 0.0 },
            weight: 1.0 / N_PARTICLES as f64,
        }
    }
}

pub struct ParticleFilter {
    rng: ThreadRng,
    pub particles: Vec<State>,
}

impl ParticleFilter {
    pub fn new() -> Self {
        ParticleFilter {
            rng: rng(),
            particles: Vec::new(),
        }
    }

    pub fn init(&mut self, distance_m: f64) {
        for i in 0..N_PARTICLES {
            self.particles.push(State {
                pos: Point2D {
                    x: distance_m
                        * f64::cos((i as f64) / (N_PARTICLES as f64) * std::f64::consts::TAU),
                    y: distance_m
                        * f64::sin((i as f64) / (N_PARTICLES as f64) * std::f64::consts::TAU),
                },
                weight: 1.0 / N_PARTICLES as f64,
            });
        }
    }

    pub fn sample(&mut self, sigma: f64) {
        let normal = Normal::new(0.0, sigma).unwrap();

        for p in self.particles.iter_mut() {
            p.pos.x += normal.sample(&mut self.rng);
            p.pos.y += normal.sample(&mut self.rng);
        }
    }

    pub fn correct(&mut self, distance_m: f64, robot: &Point2D<f64>, sigma: f64) {
        let gaussian = StatrsNormal::new(0.0, sigma).unwrap();

        let weight_sum: f64 = self
            .particles
            .iter_mut()
            .map(|p| {
                let dy = p.pos.euclidean_distance(&robot) - distance_m;
                p.weight = gaussian.pdf(dy);
                p.weight
            })
            .sum();

        for p in &mut self.particles {
            p.weight /= weight_sum + 1e-9;
        }
    }

    pub fn resample(&mut self) {
        let mut new_particles: Vec<State> = self.particles.clone();

        let r = self.rng.random_range(0.0..=1.0 / (N_PARTICLES as f64));
        let mut c = self.particles.first().unwrap().weight;
        let mut i = 1;

        for m in 0..N_PARTICLES {
            let u = r + ((m as f64) - 1.0) * 1.0 / (N_PARTICLES as f64);

            while u > c {
                i = (i + 1) % N_PARTICLES;
                c += self.particles[i].weight;
            }

            new_particles[m] = self.particles[i];
        }

        self.particles = new_particles;
    }

    pub fn mean_position(&self) -> Point2D<f64> {
        let mut mean_x = 0.0;
        let mut mean_y = 0.0;

        for particle in self.particles.iter() {
            mean_x += particle.pos.x * particle.weight;
            mean_y += particle.pos.y * particle.weight;
        }

        Point2D {
            x: mean_x,
            y: mean_y,
        }
    }
}
