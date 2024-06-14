extern crate css_color_parser;

use colors_transform::Rgb;
use image;
use image::GenericImageView;
use std::{fs, io, path::PathBuf};

use skia_safe::{
    AlphaType, Color4f, ColorType, EncodedImageFormat, ImageInfo, Paint, Rect, Surface,
};

use css_color_parser::Color as CssColor;

use crate::frame::{BruhDelta, TakeRef};

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

    pub fn into_png(&self, dir: &PathBuf, frame: usize, width: usize) -> Result<(), io::Error> {
        let mut img = dir.clone();
        img.push(format!("img{frame}.png"));

        let info = ImageInfo::new(
            (width as i32, self.pixels.len() as i32),
            ColorType::RGBA8888,
            AlphaType::Opaque,
            None,
        );

        let mut surface = Surface::new_raster(&info, None, None).unwrap();
        let canvas = surface.canvas();

        for (i, color) in self.pixels.iter().enumerate() {
            let hex = color.to_css_hex_string();
            let parsed = hex.parse::<CssColor>().unwrap();
            let color4f = Color4f::new(parsed.r as f32, parsed.g as f32, parsed.b as f32, 0.004);
            let paint = Paint::new(color4f, None);

            let x = i % width;
            let y = i / width;

            let rect = Rect::from_point_and_size((x as f32, y as f32), (1.0, 1.0));
            canvas.draw_rect(rect, &paint);
        }

        let image = surface.image_snapshot();

        if let Some(data) = image.encode(None, EncodedImageFormat::PNG, 100) {
            fs::write(img, &*data).expect("Failed to write image data to file");
        }

        Ok(())
    }

    pub fn update(&mut self, delta: &Vec<BruhDelta>) {
        let mut delta = delta.clone();
        let mut curridx = 0;

        for px in &mut self.pixels {
            match delta[curridx] {
                BruhDelta::Skip(ref mut i) => {
                    *i -= 1;

                    if *i == 0 {
                        curridx += 1;
                    }
                }
                BruhDelta::Overwrite(overwrite) => {
                    *px = overwrite;
                    curridx += 1;
                }
            }
        }
    }
}

pub fn decode_rgb(v: Vec<u8>) -> Rgb {
    let s = String::from_utf8(v).unwrap();

    Rgb::from_hex_str(&s).unwrap()
}
