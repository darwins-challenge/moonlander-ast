use moonlander_gp::Number;
use std::io::Read;
use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;
use toml;

#[derive(Debug,RustcDecodable)]
pub struct EvolutionParams {
    pub population_size: usize,
    pub trials_per_program: usize,
    pub tournament_size: usize,
    pub reproduce_weight: u32,
    pub mutate_weight: u32,
    pub crossover_weight: u32,
    pub start_position: StartPosition
}

#[derive(Debug,RustcDecodable)]
pub struct StartPosition {
    pub y: NumRange,
    pub x: NumRange,
    pub o: NumRange
}

#[derive(Debug,RustcDecodable)]
pub struct NumRange {
    pub min: Number,
    pub max: Number
}

pub fn load_params<P: AsRef<OsStr> + ?Sized>(path: &P) -> EvolutionParams {
    let mut f = File::open(Path::new(path)).ok().expect(("Error opening file"));
    let mut s = String::new();
    f.read_to_string(&mut s).ok().expect("Error reading file");
    toml::decode_str(&s).expect("Error decoding TOML file")
}
