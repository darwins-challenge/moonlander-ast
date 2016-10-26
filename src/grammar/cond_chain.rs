use super::*;
use super::super::sim::*;

/// Condition list in the form of an AST node
///
/// This is a reimplementation of `CondList`, as an enum with chained
/// nodes. This node has the same expressiveness as the `CondList`,
/// but has the advantage that it _can_ do crossover splicing between
/// parents in the current evolutionary framework.
///
/// A downside of this node type is that the programs tend to grow
/// without bounds, as there is no height control on the crossover
/// operation and crossovers tend to be done most of the time. This
/// leads to a population that quickly grows outside bounds, reducing
/// evolutionary and computational performance.
#[derive(Debug,RustcDecodable,RustcEncodable,Clone,PartialEq)]
pub enum CondChain {
    Cond(Box<Condition>, Box<Command>, Box<CondChain>),
    Stop
}

impl_astnode!(CondChain, 5,
              int Cond(cond, then, next),
              leaf Stop());

impl EvaluateToCommand for CondChain {
	fn evaluate(&self, sensor_data: &SensorData) -> Command {
        match *self {
            CondChain::Cond(ref cond, ref then, ref next) => {
                if cond.is_true(sensor_data) {
                    **then
                } else {
                    next.evaluate(sensor_data)
                }
            },
            CondChain::Stop => Command::Skip
        }
    }
}
