use std::fmt;
use moonlander_gp::RandNode;
use rand;

#[derive(Debug,RustcDecodable,RustcEncodable,Clone,PartialEq,Copy)]
pub enum Command {
	Skip,
	Left,
	Right,
	Thrust
}

impl_astnode!(Command, 1,
              Skip(),
              Left(),
              Right(),
              Thrust());

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Command::Skip => write!(f, "Skip"),
            Command::Left => write!(f, "Left"),
            Command::Right => write!(f, "Right"),
            Command::Thrust => write!(f, "Thrust")
        }
    }
}

impl RandNode for Command {
    fn rand(rng: &mut rand::Rng) -> Self {
        pick![rng,
            1, Command::Skip,
            1, Command::Left,
            1, Command::Right,
            1, Command::Thrust
        ]
    }
}

