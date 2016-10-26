use moonlander_gp::Number;

/// `SensorData` represents the information that is available for programs to
/// decide what `Command` to execute when it is evaluated, plus the values that
/// are available to the fitness function to calculate a score for the program.
#[derive(Copy,Clone,RustcEncodable)]
pub struct SensorData {
    /// Horizontal position.
    pub x:  Number,
    /// Vertical position.
    pub y:  Number,
    /// Horizontal velocity (in units per frame).
    pub vx: Number,
    /// Vertical velocity (in units per frame).
    pub vy: Number,
    /// Rotation, in radians.
    pub o:  Number,
    /// Rotational velocity (in radians per frame).
    pub w:  Number,
    /// Amount of fuel remaining.
    pub fuel: Number,
    /// Whether the lander has hit the ground.
    pub hit_ground: bool,
    /// Whether the lander has succesfully landed.
    pub landed: bool,
    /// Whether the lander is currently thrusting.
    pub thrusting: bool,
    /// Whether the lander is using the left booster.
    pub thrust_left: bool,
    /// Whether the lander is using the right booster.
    pub thrust_right: bool,
    /// If the lander hit the ground too hard and crashed, the velocity with
    /// which it hit the ground.
    pub crash_speed: Number
}

impl SensorData {
    /// Creates a builder for `SensorData`. It should be used in the following sense:
    ///
    /// ```
    /// let sensor_data = moonlander_ast::sim::SensorData::new().with_y(100.0);
    /// ```
    pub fn new () -> SensorData {
        SensorData {
            x:            0.0,
            y:            0.0,
            vx:           0.0,
            vy:           0.0,
            o:            0.0,
            w:            0.0,
            fuel:         1.0,
            hit_ground:   false,
            landed:       false,
            thrusting:    false,
            thrust_left:  false,
            thrust_right: false,
            crash_speed:  0.0,
        }
    }

    pub fn with_x(self, x: Number) -> SensorData {
        SensorData { x: x, ..self }
    }

    pub fn with_y(self, y: Number) -> SensorData {
        SensorData { y: y, ..self }
    }

    pub fn with_vx(self, vx: Number) -> SensorData {
        SensorData { vx: vx, ..self }
    }

    pub fn with_vy(self, vy: Number) -> SensorData {
        SensorData { vy: vy, ..self }
    }

    pub fn with_o(self, o: Number) -> SensorData {
        SensorData { o: o, ..self }
    }

    pub fn with_w(self, w: Number) -> SensorData {
        SensorData { w: w, ..self }
    }

    pub fn with_fuel(self, fuel: Number) -> SensorData {
        SensorData { fuel: fuel, ..self }
    }

    pub fn hit_ground(self) -> SensorData {
        SensorData { hit_ground: true, ..self }
    }

    pub fn landed(self) -> SensorData {
        SensorData { landed: true, ..self }
    }

    pub fn thrusting(self) -> SensorData {
        SensorData { thrusting: true, ..self }
    }
}
