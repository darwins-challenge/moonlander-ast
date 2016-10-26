use super::*;
use super::super::sim::*;
use moonlander_gp::{AstNode, RandNode, clone_or_replace, NodeWeights};
use rand;

#[derive(Debug,RustcDecodable,RustcEncodable,Clone,PartialEq)]
pub struct CondList(pub Vec<(Box<Condition>, Command)>);

impl AstNode for CondList {
    fn node_type(&self) -> usize { 5 }

    fn children(&self) -> Vec<&AstNode> {
        let CondList(ref decisions) = *self;
        decisions.iter()
                .map(|&(ref cond, _)| cond.as_ref() as &AstNode)
                .collect()
    }

    fn replace_child(&self, old_child: &AstNode, new_child: &mut Option<Box<AstNode>>) -> Box<AstNode> {
        let CondList(ref decisions) = *self;
        let list = decisions.iter()
                .map(|&(ref cond, ref then)| (
                        clone_or_replace(cond.as_ref(), old_child, new_child), then.clone()))
                .collect();
        Box::new(CondList(list))
    }
}

impl RandNode for CondList {
    fn rand(weights: NodeWeights, rng: &mut rand::Rng) -> Self {
        let len = rng.next_u32() % 6 + 1;
        CondList((0..len).into_iter()
                     .map(|_| (weights.gen_child(rng), Command::rand(weights.next_level(), rng)))
                     .collect())
    }
}

impl EvaluateToCommand for CondList {
	fn evaluate(&self, sensor_data: &SensorData) -> Command {
        let CondList(ref decisions) = *self;
        for d in decisions {
            if d.0.is_true(sensor_data) {
                return d.1;
            }
        }
        return Command::Skip;
    }
}
