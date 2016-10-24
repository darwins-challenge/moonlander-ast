use std::io::Write;
use rand::{Rng, StdRng};

use moonlander_gp::{AstNode,RandNode,Population,random_population};
use moonlander_gp::genetic::{evolve,Weights,Fitness};
use super::evolution_params::EvolutionParams;

pub fn run_evolution<P, F, FF, S>(params: &EvolutionParams, out: &mut Write, fitness_func: &FF, selector: &S)
    where P: AstNode+RandNode+Clone,
          F: Fitness,
          FF: Fn(&P, &mut Rng) -> F+Sync,
          S: for<'a> Fn(&'a Population<P, F>, &mut Rng) -> &'a P
{
    let mut rng = StdRng::new().unwrap();
    let mut population = random_population::<P, F, StdRng>(params.population_size, &mut rng);
    let weights = Weights {
        reproduce: params.reproduce_weight,
        mutate: params.mutate_weight,
        crossover: params.crossover_weight
    };

    loop {
        //serialize::log(&population.population);
        population.score(fitness_func, &mut rng);
        println_err!("Generation {}, best {}, average {}", population.generation, population.best_score(), population.avg_score());
        // Write outputs

        population = evolve(population, &weights, &mut rng, selector);
    }
}
