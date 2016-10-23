use std::fmt;
use super::*;
use super::super::sim::*;
use moonlander_gp::RandNode;
use rand;

#[derive(Debug,RustcDecodable,RustcEncodable,Clone,PartialEq)]
pub enum Program {
	If(Box<Condition>, Box<Program>, Box<Program>),
	Command(Box<Command>),
}

impl_astnode!(Program, 4,
              If(cond, one, two),
              Command(command));

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Program::If(ref cond, ref one, ref two) => write!(f, "({} then {} else {})", cond, one, two),
            Program::Command(ref command) => write!(f, "{}", command)
        }
    }
}

impl RandNode for Program {
    fn rand(rng: &mut rand::Rng) -> Self {
        pick![rng,
            3, Program::If(Box::new(Condition::rand(rng)), Box::new(Program::rand(rng)), Box::new(Program::rand(rng))),
            // This weight needs to be SLIGHTLY higher than the previous one, to increase the
            // chances of termination during random generation. Otherwise there's a too high chance
            // we're going to be generating Programs that contain Programs that contain Programs,
            // etc.
            4, Program::Command(Box::new(Command::rand(rng)))
        ]
    }
}

impl EvaluateToCommand for Program {
	fn evaluate(&self, sensor_data: &SensorData) -> Command {
		match *self {
			Program::If(ref condition, ref true_program, ref false_program) => {
				if (*condition).is_true(sensor_data) {
					true_program.evaluate(sensor_data)
				} else {
					false_program.evaluate(sensor_data)
				}
			},
			Program::Command(ref command) => **command
		}
	}
}

