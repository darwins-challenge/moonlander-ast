use moonlander_gp::Number;

#[derive(Clone,Copy)]
pub struct World {
    pub angular_increment: Number,
    pub gravitational_constant: Number,
    pub thrust_constant: Number,
    pub tolerance: Number,
    pub fuel_consumption: Number,
    pub max_landing_angle_rads: Number,
    pub max_landing_speed: Number
}

impl World {
    /// Creates a builder for `World`. it should be used in the following sense:
    ///
    /// ```
    /// let world = ast::simulation::World::new();
    /// ```
    pub fn new() -> World {
        World {
            angular_increment: 0.1,
            gravitational_constant: -0.5,
            thrust_constant: 0.6,
            tolerance: 0.01,
            fuel_consumption: 0.01,
            max_landing_angle_rads: 0.1,
            max_landing_speed: 0.5
        }
    }

    pub fn with_angular_increment(self, angular_increment: Number) -> World {
        World { angular_increment: angular_increment, ..self}
    }

    pub fn with_gravitational_constant(self, gravitational_constant: Number) -> World {
        World { gravitational_constant: gravitational_constant, ..self}
    }

    pub fn with_max_landing_speed(self, max_landing_speed: Number) -> World {
        World { max_landing_speed: max_landing_speed, ..self}
    }

    pub fn with_max_landing_angle_rads(self, max_landing_angle_rads: Number) -> World {
        World { max_landing_angle_rads: max_landing_angle_rads, ..self}
    }

    pub fn with_thrust_constant(self, thrust_constant: Number) -> World {
        World { thrust_constant: thrust_constant, ..self}
    }

    pub fn with_tolerance(self, tolerance: Number) -> World {
        World { tolerance: tolerance, ..self}
    }

    pub fn with_fuel_consumption(self, fuel_consumption: Number) -> World {
        World { fuel_consumption: fuel_consumption, ..self}
    }
}
