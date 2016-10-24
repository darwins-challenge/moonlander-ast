use std::fmt;
use super::*;
use super::super::sim::*;
use moonlander_gp::{AstNode, RandNode, clone_or_replace, TargetHeight};
use rand;

#[derive(Debug,RustcDecodable,RustcEncodable,Clone,PartialEq)]
pub struct DecisionList(pub Vec<(Box<Condition>, Command)>);

impl AstNode for DecisionList {
    fn node_type(&self) -> usize { 5 }

    fn children(&self) -> Vec<&AstNode> {
        let DecisionList(ref decisions) = *self;
        decisions.iter()
                .map(|&(ref cond, _)| cond.as_ref() as &AstNode)
                .collect()
    }

    fn replace_child(&self, old_child: &AstNode, new_child: &mut Option<Box<AstNode>>) -> Box<AstNode> {
        let DecisionList(ref decisions) = *self;
        let list = decisions.iter()
                .map(|&(ref cond, ref then)| (
                        clone_or_replace(cond.as_ref(), old_child, new_child), then.clone()))
                .collect();
        Box::new(DecisionList(list))
    }
}

impl fmt::Display for DecisionList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let DecisionList(ref decisions) = *self;
        for d in decisions {
            try!(writeln!(f, "{} -> {}", d.0, d.1));
        }
        Ok(())
    }
}

impl RandNode for DecisionList {
    fn rand(weights: TargetHeight, rng: &mut rand::Rng) -> Self {
        let len = rng.next_u32() % 6 + 1;
        DecisionList((0..len).into_iter()
                     .map(|_| (weights.gen_child(rng), Command::rand(weights.next_level(), rng)))
                     .collect())
    }
}

impl EvaluateToCommand for DecisionList {
	fn evaluate(&self, sensor_data: &SensorData) -> Command {
        let DecisionList(ref decisions) = *self;
        for d in decisions {
            if d.0.is_true(sensor_data) {
                return d.1;
            }
        }
        return Command::Skip;
    }
}
