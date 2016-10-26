use super::*;
use super::super::sim::*;
use moonlander_gp::{AstNode, RandNode, clone_or_replace, NodeWeights};
use rand;

/// Condition List node
///
/// This node is designed to live at the top of the program tree, and contains
/// an ordered list of `(Condition, Command)` pairs.
///
/// The first condition that matches will lead to the corresponding `Command`
/// being executed.
///
/// The advantage of this structure is that _in theory_ it will have better
/// localized crossover and mutation properties (although localized crossover by
/// splicing in the middle of the condition list is currently not possible in
/// the `moonlander_gp` framework).
///
/// Downside of this node type is that it's not an enum, and so the
/// `impl_astnode!` macro will not work for it. All traits have to be
/// implemented manually.
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
