#[derive(Debug, Default, serde::Deserialize)]
pub(crate) struct SessionConfig{
    save_result_csv: bool,
    show_plot: bool,
}