#[derive(Debug, Default, serde::Deserialize)]
pub struct BernoulliPoint {
    pub(crate) pressure : f32,
    pub(crate) velocity: f32,
    pub(crate) height: f32
}