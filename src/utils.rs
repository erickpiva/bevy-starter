use bevy::prelude::*;
use rand::{Rng, RngExt, distr::uniform::SampleUniform};

pub fn random_number<T>(min: T, max: T) -> T
where
    T: SampleUniform + PartialOrd,
{
    let mut rng = rand::rng();
    rng.random_range(min..max)
}
