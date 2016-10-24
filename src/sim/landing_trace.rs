use moonlander_gp::{Fitness,ScoreCard};
use super::SensorData;

pub type Trace = Vec<SensorData>;

/// A single trace and a score for it
#[derive(RustcEncodable)]
pub struct LandingTrace {
    pub trace: Trace,
    pub score_card: ScoreCard
}

impl LandingTrace {
    pub fn new() -> LandingTrace {
        LandingTrace { trace: vec![], score_card: ScoreCard::new(vec![]) }
    }
}

impl Fitness for LandingTrace {
    fn score_card(&self) -> &ScoreCard { &self.score_card }
}
