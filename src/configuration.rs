#[derive(Debug, serde::Deserialize)]
pub(crate) struct SessionConfig{
    pub(crate) save_result_csv: bool,
    pub(crate) show_plot: bool,
    pub(crate) solver_mode: String,
    pub(crate) inlet_file_path: String,
    pub(crate) outlet_file_path: String,
    pub(crate) streamline_file_path: String
}