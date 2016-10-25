use super::*;
use super::super::sim::*;
use moonlander_gp::{Number};
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
              leaf Constant((data n random_constant)),
              leaf Sensor(sensor),
              int Plus(left, right),
              int Minus(left, right),
              int Multiply(left, right),
              int Divide(left, right));

fn random_constant(rng: &mut rand::Rng) -> Number {
        rng.next_f32() * 100.0 - 50.0
}

impl NumericValue for Expression {
	fn num_value(&self, sensor_data: &SensorData) -> Number {
		match *self {
			Expression::Constant(value)               => value,
			Expression::Sensor(ref sensor)            => sensor.num_value(sensor_data),
			Expression::Plus(ref left, ref right)     => left.num_value(sensor_data) + right.num_value(sensor_data),
			Expression::Minus(ref left, ref right)    => left.num_value(sensor_data) - right.num_value(sensor_data),
			Expression::Multiply(ref left, ref right) => left.num_value(sensor_data) * right.num_value(sensor_data),
			Expression::Divide(ref left, ref right)   => {
                let right = right.num_value(sensor_data);
                // Not correct but at least it prevents a division by zero
                if right == 0.0 {
                    left.num_value(sensor_data)
                } else {
                    left.num_value(sensor_data) / right
                }
            }
		}
	}
}

