use std::fmt;
use super::*;
use moonlander_gp::RandNode;
use rand;

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
              True(),
              False(),
              Not(cond),
              Or(left, right),
              And(left, right),
              Less(left, right),
              LessEqual(left, right),
              Equal(left, right),
              GreaterEqual(left, right),
              Greater(left, right));


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

impl RandNode for Condition {
    fn rand(rng: &mut rand::Rng) -> Self {
        pick![rng,
            10, Condition::True,
            10, Condition::False,

            3,  Condition::Not(Box::new(Condition::rand(rng))),
            2,  Condition::Or(Box::new(Condition::rand(rng)), Box::new(Condition::rand(rng))),
            2,  Condition::And(Box::new(Condition::rand(rng)), Box::new(Condition::rand(rng))),

            1,  Condition::Less(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
            1,  Condition::LessEqual(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
            1,  Condition::Greater(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
            1,  Condition::GreaterEqual(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng))),
            2,  Condition::Equal(Box::new(Expression::rand(rng)), Box::new(Expression::rand(rng)))
        ]
    }
}
