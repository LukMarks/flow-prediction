#[derive(Debug, serde::Deserialize)]
pub(crate) struct BernoulliPoint {
    pressure : f32,
    velocity: f32,
    height: f32
}