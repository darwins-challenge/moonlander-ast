use moonlander_gp::Number;
use std::io::Read;
use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;
use toml;
use clap::{Arg, App};

/// Parameters to the evolution process.
#[derive(Debug,RustcDecodable)]
pub struct RunParams {
    pub output_every_gen: bool,
    pub evolution: EvolutionParams,
}

/// Parameters of the evolution itself. These are read from a TOML file.
#[derive(Debug,RustcDecodable)]
pub struct EvolutionParams {
    pub population_size: usize,
    pub trials_per_program: usize,
    pub tournament_size: usize,
    pub reproduce_weight: u32,
    pub mutate_weight: u32,
    pub crossover_weight: u32,
    pub tree_depth: usize,
    pub start_position: StartPosition
}

#[derive(Debug,RustcDecodable)]
pub struct StartPosition {
    pub y: NumRange,
    pub x: NumRange,
    pub o: NumRange
}

/// The numerical range for a randomly-generated number.
#[derive(Debug,RustcDecodable)]
pub struct NumRange {
    pub min: Number,
    pub max: Number
}

/// Parse command-line parameters.
pub fn load_params() -> RunParams {
    let matches = App::new("Genetic Programming Example Program")
        .about("Evolves a genetic algorithm based on the parameters in the example program")
        .arg(Arg::with_name("everygen")
             .short("e")
             .long("everygen")
             .help("Writes the champion of every generation to output (otherwise only better champions will be written)"))
        .arg(Arg::with_name("evofile")
             .help("TOML file with evolution parameters")
             .value_name("SCENARIO")
             .default_value("scenario.toml"))
        .get_matches();

    RunParams {
        output_every_gen: matches.is_present("everygen"),
        evolution: load_evo_params(matches.value_of("evofile").unwrap())
    }
}

pub fn load_evo_params<P: AsRef<OsStr> + ?Sized>(path: &P) -> EvolutionParams {
    println_err!("Opening scenario file {}", path.as_ref().to_str().unwrap());
    let mut f = File::open(Path::new(path)).ok().expect(("Error opening file"));
    let mut s = String::new();
    f.read_to_string(&mut s).ok().expect("Error reading file");
    toml::decode_str(&s).expect("Error decoding TOML file")
}
