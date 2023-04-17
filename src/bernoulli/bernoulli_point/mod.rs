#[derive(Debug, Default, serde::Deserialize, serde::Serialize, Copy, Clone)]
pub struct BernoulliPoint {
    pub(crate) pressure : f32,
    pub(crate) velocity: f32,
    pub(crate) height: f32
}