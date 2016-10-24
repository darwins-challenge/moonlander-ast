use std::fmt;
use super::*;
use super::super::sim::*;

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


impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Condition::True                       => write!(f, "True"),
            Condition::False                      => write!(f, "False"),

            Condition::Not(ref c)                 => write!(f, "!{}", c),
            Condition::Or(ref l, ref r)           => write!(f, "({} || {})", l, r),
            Condition::And(ref l, ref r)          => write!(f, "({} && {})", l, r),

            Condition::Less(ref l, ref r)         => write!(f, "({} < {})", l, r),
            Condition::LessEqual(ref l, ref r)    => write!(f, "({} <= {})", l, r),
            Condition::Equal(ref l, ref r)        => write!(f, "({} == {})", l, r),
            Condition::GreaterEqual(ref l, ref r) => write!(f, "({} >= {})", l, r),
            Condition::Greater(ref l, ref r)      => write!(f, "({} > {})", l, r),
        }
    }
}

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
