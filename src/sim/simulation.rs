use super::{SensorData, EvaluateToCommand, World};
use super::super::grammar::*;
use moonlander_gp::Number;
use super::super::num::abs;

/// Evaluate a program and apply the resulting command to the simulation.
pub fn evaluate_program(sensor_data: &mut SensorData, program: &EvaluateToCommand, world: &World) {
    let command = program.evaluate(sensor_data);
    apply_command(sensor_data, command, world);
}

/// Apply a command to the simulation.
pub fn apply_command(sensor_data: &mut SensorData, command: Command, world: &World) {
    if sensor_data.hit_ground { return; }

    let angular_multiplier: Number = match command {
        Command::Left  =>  1.0,
        Command::Right => -0.1,
        _              =>  0.0
    };
    sensor_data.w += angular_multiplier * world.angular_increment;
    sensor_data.o += sensor_data.w;

    sensor_data.thrust_left = command == Command::Left;
    sensor_data.thrust_right = command == Command::Right;

    let thrust_multiplier: Number = match command {
        Command::Thrust => { if sensor_data.fuel > 0.0 { 1.0 } else { 0.0 } },
        _               => 0.0
    };
    let acceleration = thrust_multiplier * world.thrust_constant;
    let ax = -acceleration * sensor_data.o.sin();
    let ay =  acceleration * sensor_data.o.cos() + world.gravitational_constant;
    sensor_data.vx += ax;
    sensor_data.vy += ay;
    sensor_data.x += sensor_data.vx;
    sensor_data.y += sensor_data.vy;

    sensor_data.fuel -= match command {
        Command::Thrust => world.fuel_consumption,
        _               => 0.0
    };
    sensor_data.fuel = if sensor_data.fuel > 0.0 { sensor_data.fuel } else { 0.0 };

    let down = sensor_data.y < world.tolerance || sensor_data.y.is_nan();
    let upright = abs(sensor_data.o) < world.max_landing_angle_rads;
    let crashed = abs(sensor_data.vy) > world.max_landing_speed;
    if down && crashed {
        sensor_data.crash_speed = abs(sensor_data.vy);
    }

    sensor_data.hit_ground = down;
    sensor_data.landed     = down && upright && !crashed;

    // No thrusting anymore if we touched down, it looks weird in the animation :)
    sensor_data.thrusting = !down && match command {
        Command::Thrust => true,
        _               => false
    }
}
