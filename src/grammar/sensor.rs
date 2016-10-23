use std::fmt;
use moonlander_gp::{AstNode, RandNode, Number};
use rand;
use super::super::sim::*;


#[derive(Debug,RustcDecodable,RustcEncodable,Clone,PartialEq,Copy)]
pub enum Sensor {
      X,
	  Y,
      Vx,
	  Vy,
      O,
      W,
      Fuel,
}

impl AstNode for Sensor {
    fn node_type(&self) -> usize { 0 }

    fn children(&self) -> Vec<&AstNode> {
        vec![]
    }

    fn replace_child(&self, _: &AstNode, _: &mut Option<Box<AstNode>>) -> Box<AstNode> {
        Box::new(self.clone())
    }
}

impl fmt::Display for Sensor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Sensor::Y    => write!(f, "Y"),
            Sensor::Vy   => write!(f, "Vy"),
            Sensor::Fuel => write!(f, "Fuel"),
            Sensor::X    => write!(f, "X"),
            Sensor::Vx   => write!(f, "Vx"),
            Sensor::O    => write!(f, "O"),
            Sensor::W    => write!(f, "W")
        }
    }
}

impl RandNode for Sensor {
    fn rand(rng: &mut rand::Rng) -> Self {
        pick![rng,
            1, Sensor::Y,
            1, Sensor::Vy,
            1, Sensor::Fuel,
            1, Sensor::X,
            1, Sensor::Vx,
            1, Sensor::O,
            1, Sensor::W
            ]
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

