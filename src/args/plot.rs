use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct Plot {
    /// The plot strings
    pub plot_string: Vec<String>,
}
