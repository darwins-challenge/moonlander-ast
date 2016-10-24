use std::fmt;
use super::*;
use super::super::sim::*;

#[derive(Debug,RustcDecodable,RustcEncodable,Clone,PartialEq)]
pub enum Program {
	If(Box<Condition>, Box<Program>, Box<Program>),
	Command(Box<Command>),
}

impl_astnode!(Program, 4,
              int If(cond, one, two),
              leaf Command(command));

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Program::If(ref cond, ref one, ref two) => write!(f, "({} then {} else {})", cond, one, two),
            Program::Command(ref command) => write!(f, "{}", command)
        }
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

