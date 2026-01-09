use std::{
    error::Error,
    io::{Read, Write},
};

use clap::Parser;
use image::{
    DynamicImage, GenericImageView, Rgba,
    imageops::{ColorMap, colorops},
};
use palette::{IntoColor, Oklaba, Srgba};

mod cli;

const U8_BRAILLE_MAP: [u8; 8] = [0, 3, 1, 4, 2, 5, 6, 7];
fn u8_to_braille(byte: u8) -> char {
    char::from_u32((byte as u32) + 0x2800).unwrap()
}

#[derive(Clone, Copy)]
pub struct OklabThreshold {
    pub threshold: f32,
}

/// turns rgba images to grayscale based on threshold value for lightness
impl ColorMap for OklabThreshold {
    type Color = Rgba<u8>;

    #[inline(always)]
    fn index_of(&self, color: &Self::Color) -> usize {
        let srgba_color = Srgba::from(color.0).into_linear();
        let oklab: Oklaba = srgba_color.into_color();
        if oklab.l >= self.threshold { 1 } else { 0 }
    }

    fn map_color(&self, color: &mut Self::Color) {
        let new_color = 0xFF * self.index_of(color) as u8;
        let luma = &mut color.0;
        luma[0] = new_color;
        luma[1] = new_color;
        luma[2] = new_color;
        luma[3] = 0xFF;
    }
}

fn image_to_braille(
    img: &DynamicImage,
    cols: u32,
    threshold: f32,
    invert: bool,
    dither: bool,
) -> (Vec<u8>, (u32, u32)) {
    let (width, height) = img.dimensions();

    let rows = ((height * cols) / width / 2) & !1;

    let horizontal_dots = cols * 2;
    let vertical_dots = rows * 4;

    let mut img = img
        .resize_to_fill(
            horizontal_dots,
            vertical_dots,
            image::imageops::FilterType::Nearest,
        )
        .into_rgba8();

    if dither {
        colorops::dither(&mut img, &OklabThreshold { threshold });
    }

    let mut braillable_bytes = vec![0u8; (cols * rows) as usize];

    for (x, y, pixel) in img.enumerate_pixels() {
        let mut lightness = (pixel.0[0] as f32) / 255.0;
        if !dither {
            let srgba_color = Srgba::from(pixel.0).into_linear();
            let oklab: Oklaba = srgba_color.into_color();
            lightness = oklab.l;
        }
        let has_dot = (invert && lightness <= threshold) || (!invert && lightness >= threshold);
        if has_dot {
            let braile_index_x = x / 2;
            let braile_index_y = y / 4;
            let braile_byte =
                &mut braillable_bytes[(braile_index_y * cols + braile_index_x) as usize];
            let bit_index = (y % 4) * 2 + (x % 2);
            *braile_byte |= 1 << U8_BRAILLE_MAP[bit_index as usize];
        }
    }

    (braillable_bytes, (cols, rows))
}

fn write_braille(braillable_bytes: Vec<u8>, cols: usize) -> std::io::Result<()> {
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    for (i, byte) in braillable_bytes.iter().enumerate() {
        if i % cols == 0 && i != 0 {
            writeln!(lock)?;
        }
        write!(lock, "{}", u8_to_braille(*byte))?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Cli::parse();
    let img = {
        if args.image_path == "-" {
            let mut buffer = vec![];
            std::io::stdin().read_to_end(&mut buffer)?;
            image::load_from_memory(&buffer)?
        } else {
            image::open(args.image_path)?
        }
    };
    let (braillable_bytes, (cols, _rows)) = image_to_braille(
        &img,
        args.column_width,
        args.threshold,
        args.invert,
        args.dither,
    );
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
