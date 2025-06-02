use rand::rng;
use rand::rngs::ThreadRng;
use rand_distr::{Distribution, Normal};
use statrs::distribution::{Continuous, Normal as StatrsNormal};

const N_PARTICLES: usize = 10;

#[derive(Clone, Copy, Debug)]
pub struct State {
    pos: [f64; 2],
    weight: f64,
}

impl State {
    pub fn new() -> Self {
        State {
            pos: [0.0; 2],
            weight: 1.0 / N_PARTICLES as f64,
        }
    }
}

pub struct ParticleFilter {
    rng: ThreadRng,
    particles: [State; N_PARTICLES],
}

impl ParticleFilter {
    pub fn new() -> Self {
        ParticleFilter {
            rng: rng(),
            particles: [State::new(); N_PARTICLES],
        }
    }

    pub fn sample(&mut self) {
        let normal = Normal::new(0.0, 3.0).unwrap();

        for p in self.particles.iter_mut() {
            p.pos[0] += normal.sample(&mut self.rng);
            p.pos[1] += normal.sample(&mut self.rng);
        }
    }

    pub fn correct(&mut self, distance_m: f64) {
        let gaussian = StatrsNormal::new(0.0, 2.0).unwrap();
        let mut weight_sum = 1e-9;

        for p in self.particles.iter_mut() {
            let y = p.pos[0].hypot(p.pos[1]);
            let dy = y - distance_m;

            p.weight = gaussian.pdf(dy);

            weight_sum += p.weight;
        }

        for p in self.particles.iter_mut() {
            p.weight /= weight_sum;
        }
    }
}
