//! Moonlander runner that will use `DecisionList` nodes to evaluate
//!
//! DecisionLists are like regular programs, but an ordered
//! list of (condition, command) pairs (as opposed to nested IF
//! statements).
extern crate moonlander_gp;
extern crate moonlander_ast;
extern crate rand;

use moonlander_gp::genetic::tournament_selection;
use moonlander_ast::{evolution, fitness, initial_state, evolution_params};
use moonlander_ast::sim::World;
use moonlander_ast::grammar::DecisionList;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let params = evolution_params::load_params(&args[1]);
    let world = World::new(); // Default settings

    let situation_fn = |rng: &mut rand::Rng| initial_state::vertical_landing(rng);

    evolution::run_evolution::<DecisionList, _, _, _>(
        &params,
        &mut std::io::stdout(),
        &|prog, rng| fitness::score_lander_multi(params.trials_per_program,
                                                 prog,
                                                 rng,
                                                 &situation_fn,
                                                 &world),
        &|pop, rng| tournament_selection(params.tournament_size, pop, rng));
}
