use crate::r#mod::BernoulliPoint;
use crate::r#mod::SystemProperties;

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

        let exit_velocity: f32 = f32::sqrt(((1.0/properties.density)*(entry_point.pressure-exit_point.pressure) +
        properties.gravity_acceleration * (properties.fluid_column_height+entry_point.height-exit_point.height))*2.0+
        entry_point.velocity.powi(2));

        return exit_velocity
    }
}

impl BernoulliSolver {
    fn get_exit_height(entry_point: BernoulliPoint, exit_point: BernoulliPoint,
                       properties: SystemProperties) -> f32 {

        let exit_height:f32 = (properties.fluid_column_height+entry_point.height) +
            (1.0/properties.gravity_acceleration)*(((entry_point.pressure-exit_point.pressure)/properties.density)+
            (entry_point.velocity.powi(2) - exit_point.velocity.powi(2))/2.0);

        return exit_height
    }
}

impl BernoulliSolver {
    pub fn solve(solver_mode: String, entry_point: BernoulliPoint, exit_point: BernoulliPoint,
             properties: SystemProperties) -> f32{
        // let result:f32 = match solver_mode.to_lowercase(){
        //     "pressure".parse().unwrap() => BernoulliSolver::get_exit_pressure(entry_point, exit_point, properties),
        //     "velocity".parse().unwrap() => BernoulliSolver::get_exit_velocity(entry_point, exit_point, properties),
        //     "height".parse().unwrap() => BernoulliSolver::get_exit_height(entry_point, exit_point, properties),
        //     _ => println!("Solver Invalid. Please use of the following options: Pressure;\nVelocity;\nHeight.")
        // };
        return 0.0;//result;
    }
}