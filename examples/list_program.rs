//! Moonlander runner that will use `CondList` nodes to evaluate
//!
//! CondLists are like regular programs, but an ordered
//! list of (condition, command) pairs (as opposed to nested IF
//! statements).
extern crate moonlander_gp;
extern crate moonlander_ast;
extern crate rand;

use moonlander_gp::genetic::tournament_selection;
use moonlander_ast::fitness;
use moonlander_ast::run::{run_evolution, load_params};
use moonlander_ast::sim::{World, initial_state};
use moonlander_ast::grammar::CondList;


fn main() {
    let params = load_params();
    let world = World::new(); // Default settings

    let situation_fn = |rng: &mut rand::Rng| initial_state::from_params(&params.evolution, rng);

    run_evolution::<CondList, _, _, _>(
        &params,
        &mut std::io::stdout(),
        &|prog, rng| fitness::score_lander_multi(params.evolution.trials_per_program,
                                                 prog,
                                                 rng,
                                                 &situation_fn,
                                                 &world),
        &|pop, rng| tournament_selection(params.evolution.tournament_size, pop, rng));
}
