use rand_chacha::ChaCha8Rng;
use rand_core::{RngCore, SeedableRng};
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;
use vanth_derive::Vanth;

#[derive(Clone, Debug, Deserialize, Serialize, Vanth)]
pub struct Rng {
    inner: ChaCha8Rng,
}

pub fn rng_new() -> Rng {
    Rng {
        inner: ChaCha8Rng::from_seed([0u8; 32]),
    }
}

pub fn rng_from_seed(seed: [u8; 32]) -> Rng {
    Rng {
        inner: ChaCha8Rng::from_seed(seed),
    }
}

pub fn rng_set_stream(rng: &mut Rng, stream: u64) {
    rng.inner.set_stream(stream);
}

pub fn rng_next_u32(rng: &mut Rng) -> u32 {
    rng.inner.next_u32()
}

pub fn rng_next_u64(rng: &mut Rng) -> u64 {
    rng.inner.next_u64()
}

pub fn rng_fill_bytes(rng: &mut Rng, dest: &mut [u8]) {
    rng.inner.fill_bytes(dest)
}

pub fn rng_gen_f32(rng: &mut Rng) -> f32 {
    rng_next_u32(rng) as f32 / u32::MAX as f32
}

pub fn rng_gen_gaussian(rng: &mut Rng, mean: f32, std_dev: f32) -> f32 {
    let uniform_for_radius_calc = rng_gen_f32(rng);
    let uniform_for_angle = rng_gen_f32(rng);
    let radius = (-2.0 * (1.0 - uniform_for_radius_calc).ln()).sqrt();
    let theta = 2.0 * PI * uniform_for_angle;
    mean + std_dev * radius * theta.cos()
}

pub trait Varo {
    /// Produce a random instance of `Self` using the provided RNG.
    fn next(digest: &mut Rng) -> Self;
}

#[derive(Clone, Debug, Deserialize, Serialize, Vanth)]
pub struct Distribution {
    /// 0th element is the mean, 1st is the variance etc.
    pub moments: Vec<f32>,
}

impl Distribution {
    pub fn sample(&self, digest: &mut Rng) -> f32 {
        if self.moments.is_empty() {
            rng_gen_f32(digest)
        } else if self.moments.len() == 1 {
            rng_gen_gaussian(digest, self.moments[0], 1.0)
        } else {
            rng_gen_gaussian(digest, self.moments[0], self.moments[1].sqrt())
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Vanth)]
pub struct Score {
    pub value: f32,
}

impl From<f32> for Score {
    fn from(value: f32) -> Self {
        Score { value }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Vanth)]
pub struct OptimizationResult {
    /// List of pairs of evaluation score and Rng used to generate the value.
    pub values: Vec<(Rng, f32)>,
}

pub fn optimize<T: Varo>(evaluator: impl Fn(T) -> Score, rng: &mut Rng, rounds: u32) -> OptimizationResult {
    let mut values: Vec<(Rng, f32)> = Vec::with_capacity(rounds as usize);
    for i in 0..rounds {
        let mut child = rng.clone();
        rng_set_stream(&mut child, i as u64);
        let t = T::next(&mut child);
        let score = evaluator(t).value;
        values.push((child, score));
    }
    values.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    OptimizationResult { values }
}
