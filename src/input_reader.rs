use serde::Deserialize;
use crate::bernoulli_point::BernoulliPoint;
use crate::system_properties::SystemProperties;
use crate::configuration::SessionConfig;


pub(crate)fn get_inlet_bernoulli_point() -> BernoulliPoint{

    let the_file = "../input/inlet.json";
    let inlet_stream_line_points: BernoulliPoint = serde_json::from_str(the_file)
        .expect("JSON was not well-formatted");

    return inlet_stream_line_points
}

pub(crate)fn get_outlet_bernoulli_point() -> BernoulliPoint{

    let the_file = "../input/outlet.json";
    let outlet_stream_line_points: BernoulliPoint = serde_json::from_str(the_file)
        .expect("JSON was not well-formatted");

    return outlet_stream_line_points
}

pub(crate)fn get_system_properties() -> SystemProperties {
    let the_file = "../input/system_definition.json";
    let stream_line_points: SystemProperties = serde_json::from_str(the_file)
        .expect("JSON was not well-formatted");

    return stream_line_points
}

pub(crate) fn get_session_configuration() -> SessionConfig {
    let the_file = "../config/configuration.json";
    let config: SessionConfig = serde_json::from_str(the_file)
        .expect("JSON was not well-formatted");

    return config
}