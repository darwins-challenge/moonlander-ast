use std::io::Write;
use rand::{Rng, StdRng};

use moonlander_gp::{AstNode,RandNode,Population,random_population,depth,Number};
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
        println_err!("Generation {:4}, best {:8.1}, average {:8.1}, average depth {:5.1}", population.generation, population.best_score(), population.avg_score(), average_depth(&population));
        {
            let champion = population.champion();
            output::write(&champion, out);
        }

        population = evolve(population, &weights, &mut rng, selector);
    }
}

fn average_depth<P, F>(population: &Population<P, F>) -> Number
    where P: AstNode+Clone,
          F: Fitness
{
    let total_depth : usize = population.population.iter().map(|p| depth(p)).sum();
    total_depth as Number / population.n() as Number
}
