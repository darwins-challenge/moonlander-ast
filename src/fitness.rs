//! Calculate the fitness of moon landing programs (MODIFY HERE).
use moonlander_gp::{AstNode, depth, ScoreCard, Number, Fitness};
use moonlander_gp::num::square;
use super::sim::{SensorData, World, apply_command,EvaluateToCommand, LandingTrace};
use rand::Rng;

/// Calculate the fitness of a single program.
///
/// This is done by running a simulation of the program until the moonlander
/// hits the ground, and scoring the program over the course of the simulation.
///
/// Ideally, the fitness function should be as smooth as possible to guide the
/// evolution in the direction of better solutions.
///
/// We return a full trace of all steps of the simulation so that we can
/// visualize it using the program in the `moonlander-visualization` crate. The
/// scores can be composed of multiple labeled sub-scores to gauge various parts
/// of the score of a program.
pub fn score_lander<P>(program: &P, _: &mut Rng, mut sensor_data: SensorData, world: &World) -> LandingTrace
    where P: EvaluateToCommand+AstNode
{
    let mut trace : Vec<SensorData> = Vec::with_capacity(100);
    let mut total_height: Number = 0.;
    let mut total_fuel: Number = 0.;
    let mut total_o : Number = 0.;

    trace.push(sensor_data);
    while !sensor_data.hit_ground {
        total_height += square(sensor_data.y);
        total_o += square(sensor_data.o);
        total_fuel += square(sensor_data.fuel);

        let command = program.evaluate(&sensor_data);
        apply_command(&mut sensor_data, command, world);

        trace.push(sensor_data);
    };

    let frames = trace.len() as Number;
    LandingTrace {
        trace: trace,
        score_card: ScoreCard::new(vec![
            ("survival_bonus",   3.0 * frames),
            ("height_penalty",   -(0.01 * total_height / frames)),
            ("fuel_bonus",        (100.0 * total_fuel / frames)),
            ("hit_ground_bonus", if sensor_data.hit_ground { 10.0 } else { 0.0 }),
            ("crash_penalty",    sensor_data.crash_speed),
            ("success_bonus",    if sensor_data.landed { 10000.0 } else { 0.0 }),
            ("rotation_penalty",  total_o * -1.0),
            ("end_rotation_penalty",  square(sensor_data.o) * -100.0),
            ("complexity_pentalty", square(depth(program) as f32) * -0.001)
        ])
    }
}

/// Score the lander multiple times and combine the scores.
///
/// Returns the _best_ trace and the _average_ score.
pub fn score_lander_multi<P, G>(n: usize, program: &P, rng: &mut Rng, sensor_data_fn: G, world: &World) -> LandingTrace
    where P : EvaluateToCommand+AstNode,
          G : Fn(&mut Rng) -> SensorData
{
    let mut best_trace = LandingTrace::new();
    let mut total_score = ScoreCard::new(vec![]);
    for _ in 0..n {
        let initial = sensor_data_fn(rng);
        let trace = score_lander(program, rng, initial, world);
        total_score += trace.score_card();
        if trace.score_card() > best_trace.score_card() || best_trace.score_card().is_empty() {
            best_trace = trace;
        }
    }

    let average_score = total_score / (n as Number);

    LandingTrace { trace: best_trace.trace, score_card: average_score }
}
