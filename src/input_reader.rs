use serde::Deserialize;

fn get_bernoulli_point() -> BernoulliPoints{

    let the_file = "../input/stream_line_property.json";
    let stream_line_points: BernoulliPoints = serde_json::from_str(the_file)
        .expect("JSON was not well-formatted");

    return stream_line_points
}

fn get_system_properties()