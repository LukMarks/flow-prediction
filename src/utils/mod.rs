use crate::bernoulli::bernoulli_point::BernoulliPoint;
use serde;
use std::fs;


pub(crate) fn show_result(result: &BernoulliPoint){
    let result_as_string = bernoulli_point_to_string(result);
    println!("Exit Bernoulli Point:\n{}", result_as_string)
}

pub(crate) fn save_result_data(result_file_path: &str, result: &BernoulliPoint){
    let result_as_string = bernoulli_point_to_string(result);
    fs::write(result_file_path, result_as_string).expect("Unable to write file");
}

fn bernoulli_point_to_string(bernoulli_point: &BernoulliPoint)->String{
    let result_as_string = serde_json::to_string(&bernoulli_point).unwrap();
    return result_as_string;
}