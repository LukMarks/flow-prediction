use serde::Deserialize;
use crate::bernoulli_point::BernoulliPoint;
use crate::system_properties::SystemProperties;

fn get_bernoulli_point() -> BernoulliPoint{

    let the_file = "../input/stream_line_property.json";
    let stream_line_points: BernoulliPoint = serde_json::from_str(the_file)
        .expect("JSON was not well-formatted");

    return stream_line_points
}

fn get_system_properties() -> SystemProperties {
    let the_file = "../input/system_definition.json";
    let stream_line_points: SystemProperties = serde_json::from_str(the_file)
        .expect("JSON was not well-formatted");

    return stream_line_points
}