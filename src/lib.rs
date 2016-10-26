//! Models the moonlander problem using genetic programming.
//!
//! The crate contains the AST node, fitness calculations,
//! and reporting code (to interface with a web-based visualizer
//! in another crate).
//!
//! The code is exercised using the examples in the `examples/`
//! directory.

#[macro_use] extern crate moonlander_gp;
extern crate toml;
extern crate clap;
extern crate rand;
extern crate rustc_serialize;

#[macro_use] pub mod run;
pub mod grammar;
pub mod sim;
pub mod num;
pub mod fitness;
