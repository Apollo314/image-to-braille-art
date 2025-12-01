use std::{error::Error, io::Write, path::Path};

use clap::Parser;
use image::GenericImageView;
use palette::{IntoColor, Oklaba, Srgba};

mod cli;

const U8_BRAILLE_MAP: [u8; 8] = [0, 3, 1, 4, 2, 5, 6, 7];
fn u8_to_braille(byte: u8) -> char {
    char::from_u32((byte as u32) + 0x2800).unwrap()
}

fn image_to_braille(
    input_path: &Path,
    cols: u32,
    threshold: f32,
    invert: bool,
) -> Result<(Vec<u8>, (u32, u32)), Box<dyn Error>> {
    let img = image::open(input_path)?;
    let (width, height) = img.dimensions();

    let rows = ((height * cols) / width / 2) & !1;

    let horizontal_dots = cols * 2;
    let vertical_dots = rows * 4;

    let img = img.resize_to_fill(
        horizontal_dots,
        vertical_dots,
        image::imageops::FilterType::Nearest,
    );

    let mut braillable_bytes = vec![0u8; (cols * rows) as usize];

    for (x, y, pixel) in img.pixels() {
        let srgba_color = Srgba::from(pixel.0).into_linear();
        let oklab_pixel: Oklaba = srgba_color.into_color();

        if (invert && oklab_pixel.l <= threshold) || (!invert && oklab_pixel.l >= threshold) {
            let braile_index_x = x / 2;
            let braile_index_y = y / 4;
            let braile_byte =
                &mut braillable_bytes[(braile_index_y * cols + braile_index_x) as usize];
            let bit_index = (y % 4) * 2 + (x % 2);
            *braile_byte |= 1 << U8_BRAILLE_MAP[bit_index as usize];
        }
    }

    Ok((braillable_bytes, (cols, rows)))
}

fn write_braille(braillable_bytes: Vec<u8>, cols: usize) -> std::io::Result<()> {
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    for (i, byte) in braillable_bytes.iter().enumerate() {
        if i % cols == 0 && i != 0 {
            writeln!(lock, "")?;
        }
        write!(lock, "{}", u8_to_braille(*byte))?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Cli::parse();
    let (braillable_bytes, (cols, _rows)) = image_to_braille(
        &args.image_path,
        args.column_width,
        args.threshold,
        args.invert,
    )?;
    write_braille(braillable_bytes, cols as usize)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_braille() {
        assert_eq!(u8_to_braille(0b00000000), '⠀', "blank braille failed");
        assert_eq!(u8_to_braille(0b00000001), '⠁', "braille dots-1 failed");
        assert_eq!(u8_to_braille(0b01000000), '⡀', "braille dots-7 failed");
    }
}
