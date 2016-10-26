//! Common evolution-running code between all examples.

#[macro_use]
mod println_err;

mod evolution;
pub use self::evolution::run_evolution;

mod evolution_params;
pub use self::evolution_params::{load_params, EvolutionParams, NumRange};

mod output;
pub use self::output::write;
