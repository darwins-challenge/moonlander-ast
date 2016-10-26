//! Routines to generate starting positions for the moonlander.

use moonlander_gp::Number;
use super::SensorData;
use super::super::run::{EvolutionParams, NumRange};
use rand;

/// Set up a vertical landing state.
pub fn vertical_landing(rng: &mut rand::Rng) -> SensorData {
    SensorData::new()
        .with_x(0.0)
        .with_y(rng.next_f32() * 400.0 + 50.0)
        .with_o(0.0)
}

/// Create an initial starting state for the lander from the given parameters.
pub fn from_params(params: &EvolutionParams, rng: &mut rand::Rng) -> SensorData {
    SensorData::new()
        .with_x(num_from_range(&params.start_position.x, rng))
        .with_y(num_from_range(&params.start_position.y, rng))
        .with_o(num_from_range(&params.start_position.o, rng))
}

fn num_from_range(range: &NumRange, rng: &mut rand::Rng) -> Number {
    range.min + rng.next_f32() * (range.max - range.min)
}
