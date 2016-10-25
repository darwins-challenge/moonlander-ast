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

