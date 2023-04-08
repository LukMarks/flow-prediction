#[derive(Debug, Default, serde::Deserialize)]
pub(crate) struct SystemProperties{
    density: f32,
    fluid_column_height: f32,
    gravity_acceleration: f32
}