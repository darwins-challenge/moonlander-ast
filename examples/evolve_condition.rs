//! Moonlander runner that will use only `Condition` nodes to evaluate
//!
//! If the condition evaluates to True, the moonlander will thrust.
//! If the condition evaluates to False, it will do nothing.
extern crate moonlander_gp;
extern crate moonlander_ast;
extern crate rand;

use moonlander_gp::genetic::tournament_selection;
use moonlander_ast::{evolution, fitness, initial_state, evolution_params};
use moonlander_ast::sim::World;
use moonlander_ast::grammar::Condition;


fn main() {
    let params = evolution_params::load_params();
    let world = World::new(); // Default settings

    let situation_fn = |rng: &mut rand::Rng| initial_state::from_params(&params.evolution, rng);

    evolution::run_evolution::<Condition, _, _, _>(
        &params,
        &mut std::io::stdout(),
        &|prog, rng| fitness::score_lander_multi(params.evolution.trials_per_program,
                                                 prog,
                                                 rng,
                                                 &situation_fn,
                                                 &world),
        &|pop, rng| tournament_selection(params.evolution.tournament_size, pop, rng));
}
