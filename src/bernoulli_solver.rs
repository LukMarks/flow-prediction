#[derive(Debug, Deserialize)]
struct BernoulliSolver {
    entry_point: BernoulliPoint,
    exit_point: BernoulliPoint,
    stream_line_properties: SystemProperties
}

impl BernoulliSolver {
    fn get_exit_speed() -> u16 {
        let exit_speed: u16 = sqrt(self::stream_line_properties.gravity_acceleration *
            self::stream_line_properties.fluid_column_height);
        return exit_speed
    }
}