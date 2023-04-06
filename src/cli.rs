#[derive(Debug, clap::Parser)]
pub struct Args {
    /// URL or filename of an image to calculate yakudo score
    pub url_or_file: String,
}
