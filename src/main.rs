mod bernoulli_point;
mod input_reader;
mod system_properties;
mod bernoulli_solver;
mod configuration;

use crate::bernoulli_point::BernoulliPoint;
use crate::configuration::SessionConfig;
use crate::system_properties::SystemProperties;
use crate::input_reader::{get_session_configuration, get_inlet_bernoulli_point,
                          get_outlet_bernoulli_point, get_system_properties};

use crate::bernoulli_solver::BernoulliSolver;


fn main() {

    let session_config: SessionConfig = get_session_configuration();
    let stream_properties:SystemProperties = get_system_properties();

    let streamline_entry:BernoulliPoint = get_inlet_bernoulli_point();
    let streamline_exit:BernoulliPoint = get_outlet_bernoulli_point();

    println!("Hello, world!");

}
