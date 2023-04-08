use crate::bernoulli_point::BernoulliPoint;
use crate::system_properties::SystemProperties;


#[derive(Debug, serde::Deserialize)]
struct BernoulliSolver {
    entry_point: BernoulliPoint,
    exit_point: BernoulliPoint,
    stream_line_properties: SystemProperties
}

impl BernoulliSolver {
    fn get_exit_speed(&self) -> f32 {
        let exit_speed: f32 = f32::sqrt(Self::stream_line_properties.gravity_acceleration *
            Self::stream_line_properties.fluid_column_height);
        return exit_speed
    }
}