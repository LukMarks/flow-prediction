use serde::__private::de::Content::F32;
use crate::bernoulli_point::BernoulliPoint;
use crate::system_properties::SystemProperties;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct BernoulliSolver {}

impl BernoulliSolver {
    fn get_exit_pressure(entry_point: BernoulliPoint, exit_point: BernoulliPoint,
                         properties: SystemProperties) -> f32{
        let exit_pressure: f32 = entry_point.pressure +
            properties.density*(0.5*(entry_point.velocity.powi(2) - exit_point.velocity.powi(2)) +
                properties.gravity_acceleration*(entry_point.height + properties.fluid_column_height - exit_point.height));

        return exit_pressure
    }
}

impl BernoulliSolver{
    fn get_exit_velocity(entry_point: BernoulliPoint, exit_point: BernoulliPoint,
                         properties: SystemProperties) -> f32{

        let exit_velocity: f32 = f32::sqrt(((1/properties.density)*(entry_point.pressure-exit_point.pressure) +
        properties.gravity_acceleration * (properties.fluid_column_height+entry_point.height-exit_point.height))*2+
        entry_point.velocity.powi(2));

        return exit_velocity
    }
}

