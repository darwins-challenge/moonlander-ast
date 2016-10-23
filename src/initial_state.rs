use super::sim::SensorData;
use rand;

pub fn vertical_landing(rng: &mut rand::Rng) -> SensorData {
    SensorData::new()
        .with_x(0.0)
        .with_y(rng.next_f32() * 400.0 + 50.0)
        .with_o(0.0)
}
