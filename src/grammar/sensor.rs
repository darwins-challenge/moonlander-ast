use std::fmt;
use moonlander_gp::Number;
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

impl_astnode!(Sensor, 0,
              leaf X(),
              leaf Y(),
              leaf Vx(),
              leaf Vy(),
              leaf O(),
              leaf W(),
              leaf Fuel());


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

