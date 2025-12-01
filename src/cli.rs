use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    pub image_path: std::path::PathBuf,

    /// Column width for output, used for scaling the image.
    #[arg(default_value_t = 60)]
    pub column_width: u32,

    /// at which lightness value of oklab, should there be a braille dot
    #[arg(long, short, default_value_t = 0.5)]
    pub threshold: f32,

    /// invert the light and dark logic for white background
    #[arg(long, short, default_value_t = false)]
    pub invert: bool,
}
