use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// path of the image or a dash(-), dash means read from stdin
    pub image_path: String,

    /// Column width for output, used for scaling the image.
    #[arg(long = "width", short = 'w')]
    pub column_width: Option<u32>,

    /// at which lightness value of oklab should there be a braille dot
    #[arg(long, short, default_value_t = 0.5)]
    pub threshold: f32,

    /// invert the light and dark logic for white background
    #[arg(long, short, default_value_t = false)]
    pub invert: bool,

    /// don't do Floyd–Steinberg dithering
    #[arg(long, short, action = clap::ArgAction::SetFalse, default_value_t = true)]
    pub dither: bool,
}
