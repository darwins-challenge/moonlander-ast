use super::*;
use super::super::sim::*;

/// A boolean condition, evaluating to either True or False.
///
/// Encodes most of the boolean expressions you would expect,
/// like `A && B`, `A || B`, plus numeric comparisons like
/// `X == Y`, `X < Y`, etc.
#[derive(Debug,RustcDecodable,RustcEncodable,Clone,PartialEq)]
pub enum Condition {
	True,
	False,

	Not(Box<Condition>),
	Or(Box<Condition>, Box<Condition>),
	And(Box<Condition>, Box<Condition>),

	Less(Box<Expression>, Box<Expression>),
	LessEqual(Box<Expression>, Box<Expression>),
	Equal(Box<Expression>, Box<Expression>),
	GreaterEqual(Box<Expression>, Box<Expression>),
	Greater(Box<Expression>, Box<Expression>),
}

impl_astnode!(Condition, 2,
              leaf True(),
              leaf False(),
              int Not(cond),
              int Or(left, right),
              int And(left, right),
              int Less(left, right),
              int LessEqual(left, right),
              int Equal(left, right),
              int GreaterEqual(left, right),
              int Greater(left, right));


impl EvaluateToCommand for Condition {
	fn evaluate(&self, sensor_data: &SensorData) -> Command {
        if self.is_true(sensor_data) { Command::Thrust } else { Command::Skip }
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
