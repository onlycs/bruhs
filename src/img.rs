extern crate css_color_parser;

use colors_transform::Rgb;
use core::slice;
use image;
use image::GenericImageView;
use itertools::{Chunk, Itertools};
use std::{
    env,
    fs::{self, OpenOptions},
    io::{self, Write},
    path::PathBuf,
};

use skia_safe::{
    AlphaType, Color4f, ColorType, EncodedImageFormat, ImageInfo, Paint, Rect, Surface,
};

use css_color_parser::Color as CssColor;

use crate::frame::TakeRef;

static TEMP_RESULT_PATH: &str = "temp.png";

#[derive(Clone, Debug, PartialEq)]
pub struct Bruh {
    pub pixels: Vec<Rgb>,
}

impl Bruh {
    pub fn parse_rgb(path: PathBuf) -> Result<Self, io::Error> {
        let img = image::open(&path).expect("File not found!");

        let mut pixels = vec![];

        for (_, _, rgba) in img.pixels() {
            let brgba = rgba.0;
            let rgb = Rgb::from(brgba[0] as f32, brgba[1] as f32, brgba[2] as f32);

            pixels.push(rgb);
        }

        let bruh = Bruh { pixels };

        Ok(bruh)
    }

    pub fn diff(&self, other: &Bruh) -> usize {
        let mut diff = 0;

        for (i, pixel) in self.pixels.iter().enumerate() {
            if pixel != &other.pixels[i] {
                diff += 1;
            }
        }

        diff
    }

    pub fn encode(&self, w: usize) -> Vec<u8> {
        let mut b = Vec::with_capacity(self.pixels.len() * 6); // ascii fits in u8

        for (i, px) in self.pixels.iter().enumerate() {
            if i % w == 0 {
                b.extend(b"\n");
            }

            b.extend(px.to_css_hex_string()[1..].as_bytes())
        }

        b
    }

    pub fn decode(b: &[u8], width: usize) -> Self {
        let mut bytes = b[1..].iter().copied();
        let mut pixels = Vec::with_capacity(b.len() / 6);
        let mut i = 0;

        loop {
            if i == width {
                i = 0;
                bytes.next();
            }

            let pixel = bytes.take_ref(6);

            if pixel.len() != 6 {
                break;
            }

            let pixel = decode_rgb(pixel);
            pixels.push(pixel);

            i += 1;
        }

        Bruh { pixels }
    }
}

pub fn decode_rgb(v: Vec<u8>) -> Rgb {
    let s = String::from_utf8(v).unwrap();

    Rgb::from_hex_str(&s).unwrap()
}

// fn bruh_to_png(path: PathBuf) -> (u32, u32) {
//     let mut contents: Vec<u8> = fs::read(&path).expect("Couldn't read file.");
//     let binding: Vec<_> = contents.drain(0..8).collect();

//     let width = vec_to_u32_ne(&binding[0..4]);
//     let height = vec_to_u32_ne(&binding[4..8]);

//     let sanitized_content = String::from_utf8_lossy(&contents).replace("\n", "");

//     let result: Vec<&str> = sanitized_content
//         .as_bytes()
//         .chunks(6)
//         .map(std::str::from_utf8)
//         .collect::<Result<_, _>>()
//         .expect("Invalid UTF-8 sequence in the input string");

//     let info = ImageInfo::new(
//         (width as i32, height as i32),
//         ColorType::RGBA8888,
//         AlphaType::Opaque,
//         None,
//     );

//     let mut surface = Surface::new_raster(&info, None, None).unwrap();
//     let canvas = surface.canvas();

//     for (i, color) in result.iter().enumerate() {
//         let hex = "#".to_owned() + color;

//         let parsed_color = hex
//             .parse::<CssColor>()
//             .expect("Failed to convert Hex to RGB");
//         let color4f = Color4f::new(
//             parsed_color.r as f32,
//             parsed_color.g as f32,
//             parsed_color.b as f32,
//             0.004 as f32,
//         );
//         let paint = Paint::new(color4f, None);
//         if i == 0 {
//             println!("{:?}", paint)
//         }
//         let x = i % width as usize;
//         let y = i / width as usize;

//         let rect = Rect::from_point_and_size((x as f32, y as f32), (1.0, 1.0));
//         canvas.draw_rect(rect, &paint);
//     }

//     let image = surface.image_snapshot();

//     if let Some(data) = image.encode(None, EncodedImageFormat::PNG, 100) {
//         fs::write(TEMP_RESULT_PATH, &*data).expect("Failed to write image data to file");
//     }

//     return (width, height);
// }
