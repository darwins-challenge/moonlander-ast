use super::super::grammar::*;
use super::SensorData;
use moonlander_gp::Number;

/// `Evaluate` returns a specific `ast::structure::Command` to execute, depending on `ast::data::SensorData`
pub trait EvaluateToCommand {
	fn evaluate(&self, sensor_data: &SensorData) -> Command;
}

/// The numeric value of an `ast::structure::Expression`
pub trait NumericValue {
	fn num_value(&self, sensor_data: &SensorData) -> Number;
}

/// The truth value of an `ast::structure::Condition`
pub trait BooleanValue {
	fn is_true(&self, sensor_data: &SensorData) -> bool;
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

impl EvaluateToCommand for DecisionList {
	fn evaluate(&self, sensor_data: &SensorData) -> Command {
        let DecisionList(ref decisions) = *self;
        for d in decisions {
            if d.0.is_true(sensor_data) {
                return d.1;
            }
        }
        return Command::Skip;
    }
}

impl EvaluateToCommand for Condition {
	fn evaluate(&self, sensor_data: &SensorData) -> Command {
        if self.is_true(sensor_data) { Command::Thrust } else { Command::Skip }
    }
}

impl NumericValue for Expression {
	fn num_value(&self, sensor_data: &SensorData) -> Number {
		match *self {
			Expression::Constant(value)               => value,
			Expression::Sensor(ref sensor)            => sensor.num_value(sensor_data),
			Expression::Plus(ref left, ref right)     => left.num_value(sensor_data) + right.num_value(sensor_data),
			Expression::Minus(ref left, ref right)    => left.num_value(sensor_data) - right.num_value(sensor_data),
			Expression::Multiply(ref left, ref right) => left.num_value(sensor_data) * right.num_value(sensor_data),
			Expression::Divide(ref left, ref right)   => left.num_value(sensor_data) / right.num_value(sensor_data)
		}
	}
}

impl NumericValue for Sensor {
    fn num_value(&self, sensor_data: &SensorData) -> Number {
        match *self {
            Sensor::X    => sensor_data.x,
            Sensor::Y    => sensor_data.y,
            Sensor::Vx   => sensor_data.vx,
            Sensor::Vy   => sensor_data.vy,
            Sensor::O    => sensor_data.o,
            Sensor::W    => sensor_data.w,
            Sensor::Fuel => sensor_data.fuel,
        }
    }
}

impl BooleanValue for Condition {
	fn is_true(&self, sensor_data: &SensorData) -> bool {
		match *self {
			Condition::True                              => true,
			Condition::False                             => false,
			Condition::Not(ref condition)                => !condition.is_true(sensor_data),
			Condition::Or(ref left, ref right)           => left.is_true(sensor_data) || right.is_true(sensor_data),
			Condition::And(ref left, ref right)          => left.is_true(sensor_data) && right.is_true(sensor_data),
			Condition::Less(ref left, ref right)         => left.num_value(sensor_data) <  right.num_value(sensor_data),
			Condition::LessEqual(ref left, ref right)    => left.num_value(sensor_data) <= right.num_value(sensor_data),
			Condition::Equal(ref left, ref right)        => left.num_value(sensor_data) == right.num_value(sensor_data),
			Condition::GreaterEqual(ref left, ref right) => left.num_value(sensor_data) >= right.num_value(sensor_data),
			Condition::Greater(ref left, ref right)      => left.num_value(sensor_data) >  right.num_value(sensor_data),
		}
	}
}
