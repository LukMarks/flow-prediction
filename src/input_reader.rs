use crate::bernoulli::bernoulli_point::BernoulliPoint;
use crate::bernoulli::flow_properties::SystemProperties;
use crate::configuration::SessionConfig;


use std::fs;

pub(crate) fn get_surface_bernoulli_point(surface_file_path: &str) -> BernoulliPoint{

    let surface_file_content =  fs::read_to_string(surface_file_path)
        .expect("Should have been able to read the file");

    let surface_stream_line_points: BernoulliPoint = serde_json::from_str(&*surface_file_content)
        .expect("JSON was not well-formatted");

    return surface_stream_line_points
}

pub(crate) fn get_system_properties(streamline_file_path: &str) -> SystemProperties {


    let system_properties_file_content =  fs::read_to_string(streamline_file_path)
        .expect("Should have been able to read the file");

    let streamline_properties: SystemProperties = serde_json::from_str(&*system_properties_file_content)
        .expect("JSON was not well-formatted");

    return streamline_properties
}

pub(crate) fn get_session_configuration(configuration_file_path: &str) -> SessionConfig {

    let config_file_content =  fs::read_to_string(configuration_file_path)
        .expect("Should have been able to read the file");

    let config: SessionConfig = serde_json::from_str(&*config_file_content)
        .expect("JSON was not well-formatted");

    return config
}