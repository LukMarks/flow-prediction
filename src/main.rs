mod bernoulli;
mod configuration;
mod utils;
pub mod input_reader;

use std::env;
use crate::configuration::SessionConfig;
use crate::input_reader::{get_session_configuration, get_surface_bernoulli_point,
                          get_system_properties};

use crate::bernoulli::bernoulli_point::BernoulliPoint;
use crate::bernoulli::bernoulli_solver::BernoulliSolver;
use crate::bernoulli::flow_properties::SystemProperties;

fn main() {
    let config_file_path: String = get_configuration_file_path();

    let session_config: SessionConfig = get_session_configuration(&config_file_path);

    let stream_properties:SystemProperties = get_system_properties(&session_config.streamline_file_path);

    let streamline_exit:BernoulliPoint = get_surface_bernoulli_point(&session_config.outlet_file_path);
    let streamline_entry:BernoulliPoint = get_surface_bernoulli_point(&session_config.inlet_file_path);

    let flow_result:BernoulliPoint = BernoulliSolver::solve(session_config.solver_mode,
    streamline_entry, streamline_exit, stream_properties);

    if session_config.show_result {
        utils::show_result(&flow_result);
    }
    if session_config.save_result {
        utils::save_result_data(&session_config.result_file_path, &flow_result);
    }

}

fn get_configuration_file_path() -> String{
    let cli_arguments: Vec<String> = env::args().collect();
    let config_file_path: String = String::from(&cli_arguments[1]);
    return config_file_path;
}
