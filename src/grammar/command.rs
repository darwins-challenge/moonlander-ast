use std::fmt;

#[derive(Debug,RustcDecodable,RustcEncodable,Clone,PartialEq,Copy)]
pub enum Command {
	Skip,
	Left,
	Right,
	Thrust
}

impl_astnode!(Command, 1,
              leaf Skip(),
              leaf Left(),
              leaf Right(),
              leaf Thrust());

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

