use std::io::{Read, Write};
use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;
use rand::{Rng, StdRng};
use toml;

use moonlander_gp::{AstNode,RandNode,Population,random_population};
use moonlander_gp::genetic::{evolve,Weights,Fitness};

#[derive(RustcDecodable)]
pub struct EvolutionParams {
    pub population_size: usize,
    pub trials_per_program: usize,
    pub tournament_size: usize,
    pub reproduce_weight: u32,
    pub mutate_weight: u32,
    pub crossover_weight: u32
}

pub fn load_params<P: AsRef<OsStr> + ?Sized>(path: &P) -> EvolutionParams {
    let mut f = File::open(Path::new(path)).ok().expect(("Error opening file"));
    let mut s = String::new();
    f.read_to_string(&mut s).ok().expect("Error reading file");
    toml::decode_str(&s).expect("Error decoding Toml file")
}

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
