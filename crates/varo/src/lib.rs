use rand_core::SeedableRng;
use serde::{Deserialize, Serialize};
use vanth_derive::Vanth;

#[derive(Clone, Debug, Deserialize, Serialize, Vanth)]
pub struct Rng {
    // TODO: RNG
}

impl Rng {
    // TODO
}

pub trait Varo {
    /// Produce a random 
    fn next(digest: &mut Rng) -> Self;
}

#[derive(Clone, Debug, Deserialize, Serialize, Vanth)]
pub struct Distribution {
    /// 0th element is the mean, 1st is the variance etc.
    pub moments: Vec<f32>,
}

impl Distribution {
    pub fn sample(digest: &mut Rng) -> f32 {
        todo!()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Vanth)]
pub struct Score {
    value: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize, Vanth)]
pub struct OptimizationResult {
    /// List of pairs of evaluation score and Rng used to generate the value.
    values: Vec<(Rng, f32)>
}

pub fn optimize<T: Varo>(evaluator: impl Fn(T) -> Score, rng: &mut Rng, rounds: u32) -> OptimizationResult {
    // TODO:
    // `for i in 0..rounds`: create a clone of `rng` and feed it `i`.
    // Call T::next() and pass it to the evaluator.
    // Return a sorted list, highest scores first.
    
    todo!()
}
