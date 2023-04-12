#[derive(Debug, Default, serde::Deserialize)]
pub(crate) struct SystemProperties{
    pub(crate) density: f32,
    pub(crate) fluid_column_height: f32,
    pub(crate) gravity_acceleration: f32
}