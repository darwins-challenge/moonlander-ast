use std::io::Write;
use rand::{Rng, StdRng};

use moonlander_gp::{AstNode,RandNode,Population,random_population};
use moonlander_gp::genetic::{evolve,Weights,Fitness};
use super::evolution_params::EvolutionParams;
use super::output;

pub fn run_evolution<P, F, FF, S>(params: &EvolutionParams, out: &mut Write, fitness_func: &FF, selector: &S)
    where P: AstNode+RandNode+Clone+::rustc_serialize::Encodable,
          F: Fitness+::rustc_serialize::Encodable,
          FF: Fn(&P, &mut Rng) -> F+Sync,
          S: for<'a> Fn(&'a Population<P, F>, &mut Rng) -> &'a P
{
    let mut rng = StdRng::new().unwrap();
    let mut population = random_population::<P, F, StdRng>(params.population_size, params.tree_depth, &mut rng);
    let weights = Weights {
        reproduce: params.reproduce_weight,
        mutate: params.mutate_weight,
        crossover: params.crossover_weight
    };

    loop {
        population.score(fitness_func, &mut rng);
        println_err!("Generation {}, best {}, average {}", population.generation, population.best_score(), population.avg_score());
        {
            let champion = population.champion();
            output::write(&champion, out);
        }

        population = evolve(population, &weights, &mut rng, selector);
    }
}
