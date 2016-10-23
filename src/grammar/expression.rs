use std::fmt;
use super::*;
use super::super::sim::*;
use moonlander_gp::{RandNode, Number};
use rand;

#[derive(Debug,RustcDecodable,RustcEncodable,Clone,PartialEq)]
pub enum Expression {
	  Constant(Number),
	  Sensor(Box<Sensor>),
	  Plus(Box<Expression>, Box<Expression>),
	  Minus(Box<Expression>, Box<Expression>),
	  Multiply(Box<Expression>, Box<Expression>),
	  Divide(Box<Expression>, Box<Expression>),
}

impl_astnode!(Expression, 3,
              Constant((data n)),
              Sensor(sensor),
              Plus(left, right),
              Minus(left, right),
              Multiply(left, right),
              Divide(left, right));

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expression::Constant(ref n)        => write!(f, "{}", n),
            Expression::Sensor(ref s)          => write!(f, "{}", s),
            Expression::Plus(ref l, ref r)     => write!(f, "({} + {})", l, r),
            Expression::Minus(ref l, ref r)    => write!(f, "({} - {})", l, r),
            Expression::Multiply(ref l, ref r) => write!(f, "({} * {})", l, r),
            Expression::Divide(ref l, ref r)   => write!(f, "({} / {})", l, r),
        }
    }
}

impl RandNode for Expression {
    fn rand(rng: &mut rand::Rng) -> Self {
        pick![rng,
            5, Expression::Constant(rng.next_f32()),
            5, Expression::Sensor(Box::new(Sensor::rand(rng))),
            1, Expression::Plus(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
            1, Expression::Minus(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
            1, Expression::Multiply(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
            1, Expression::Divide(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng)))
        ]
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

