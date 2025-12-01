use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the input image file.
    pub image_path: std::path::PathBuf,

    /// Column width for output, used for scaling the image.
    #[arg(default_value_t = 60)] // Sets the default value to 60
    pub column_width: u32,
}
