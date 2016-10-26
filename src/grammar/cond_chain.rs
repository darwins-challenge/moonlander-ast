use super::*;
use super::super::sim::*;

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
