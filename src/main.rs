mod input_reader;
mod bernoulli;
mod configuration;
mod utils;
use std::env;

use crate::configuration::SessionConfig;
use crate::input_reader::{get_session_configuration, get_inlet_bernoulli_point,
                          get_outlet_bernoulli_point, get_system_properties};

use crate::bernoulli::bernoulli_system::*;


fn main() {
    let config_file_path: String = get_configuration_file_path();

    let session_config: SessionConfig = get_session_configuration(&config_file_path);
    println!("{:?}", session_config);
    let stream_properties:SystemProperties = get_system_properties(&session_config.streamline_file_path);

    let streamline_exit:BernoulliPoint = get_outlet_bernoulli_point(&session_config.outlet_file_path);
    let streamline_entry:BernoulliPoint = get_inlet_bernoulli_point(&session_config.inlet_file_path);


    if session_config.show_plot {
        utils::save_result_plot();
    }else if session_config.save_result_csv {
        utils::save_result_data();
    }

}

fn get_configuration_file_path() -> String{
    let cli_arguments: Vec<String> = env::args().collect();
    let config_file_path: String = String::from(&cli_arguments[1]);
    return config_file_path;
}