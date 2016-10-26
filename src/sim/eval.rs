use super::super::grammar::*;
use super::SensorData;
use moonlander_gp::Number;

/// `Evaluate` returns a specific `Command` to execute, depending on `SensorData`.
pub trait EvaluateToCommand {
	fn evaluate(&self, sensor_data: &SensorData) -> Command;
}

/// The numeric value of an `Expression`.
pub trait NumericValue {
	fn num_value(&self, sensor_data: &SensorData) -> Number;
}

/// The truth value of a `Condition`.
pub trait BooleanValue {
	fn is_true(&self, sensor_data: &SensorData) -> bool;
}

