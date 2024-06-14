use colors_transform::{Color, Rgb};
use std::{io, path::PathBuf};

use crate::img::Bruh;

#[derive(Debug)]
pub enum BruhDelta {
    Skip(u32), // skip: length
    Overwrite(Rgb),
}

impl BruhDelta {
    pub fn encode(this: &Vec<Self>, capacity: usize) -> Vec<u8> {
        let mut b = Vec::with_capacity(capacity);

        for i in this {
            match i {
                BruhDelta::Skip(i) => {
                    b.extend(b"s");
                    b.extend(i.to_ne_bytes());
                }
                BruhDelta::Overwrite(col) => {
                    b.extend(b"o");
                    b.extend(col.to_css_hex_string()[1..].as_bytes());
                }
            }
        }

        b
    }
}

#[derive(Debug)]
pub enum Frame {
    Key(Bruh),
    Delta(Vec<BruhDelta>),
}

impl Frame {
    pub fn force_key(&self) -> &Bruh {
        match self {
            Frame::Delta(_) => panic!(),
            Frame::Key(bruh) => bruh,
        }
    }
}

#[derive(Debug)]
pub struct Bruhs {
    pub frames: Vec<Frame>,
    width: usize,
    height: usize,
}

fn pxdiff(a: &Rgb, b: &Rgb) -> u8 {
    let r1 = a.get_red();
    let g1 = a.get_green();
    let b1 = a.get_blue();

    let r2 = b.get_red();
    let g2 = b.get_green();
    let b2 = b.get_blue();

    let dr = r1 - r2;
    let dg = g1 - g2;
    let db = b1 - b2;

    let below = dr.powi(2) + dg.powi(2) + db.powi(2);
    let root = (below as f64).sqrt();

    root as u8
}

impl Bruhs {
    pub fn parse_gif(path: PathBuf, width: usize, height: usize) -> Result<Self, io::Error> {
        let mut pngsdir = path.clone();
        pngsdir.set_extension("pngs_tmp");

        //create dir
        std::fs::create_dir(&pngsdir)?;

        // run ffmpeg
        let output = std::process::Command::new("ffmpeg")
            .args(&[
                "-i",
                path.to_str().unwrap(),
                "-vf",
                format!("scale={}:{}", width, height).as_str(),
                format!("{}/%04d.png", pngsdir.to_str().unwrap()).as_str(),
            ])
            .output()
            .expect("failed to execute process");

        if !output.status.success() {
            panic!("ffmpeg failed: {:?}", output.status.code());
        }

        let bruh = Self::parse_dir(pngsdir.clone(), width, height)?;
        std::fs::remove_dir_all(pngsdir)?;

        Ok(bruh)
    }

    pub fn parse_dir(path: PathBuf, width: usize, height: usize) -> Result<Self, std::io::Error> {
        let mut frames = vec![];

        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            let frame = Bruh::parse_rgb(path)?;
            frames.push(Frame::Key(frame));
        }

        let mut bruh = Bruhs {
            frames,
            width,
            height,
        };

        bruh.deltify();

        Ok(bruh)
    }

    pub fn deltify(&mut self) {
        let mut iter = self.frames.iter_mut();
        let mut key = iter.next().unwrap().force_key().clone();

        for next in iter {
            let frame = next.force_key().clone();
            let diff = key.diff(&frame);

            // keep a keyframe if the difference is > 90%
            if diff > ((self.width * self.height) / 10) * 9 {
                key = frame;
                continue;
            }

            // build a delta
            let mut deltas = vec![];

            for (i, pixel) in key.pixels.iter().enumerate() {
                if pxdiff(pixel, &frame.pixels[i]) > 30 {
                    deltas.push(BruhDelta::Overwrite(frame.pixels[i]));
                    continue;
                }

                match deltas.last_mut() {
                    Some(BruhDelta::Skip(len)) => {
                        if i % self.width as usize == 0 {
                            deltas.push(BruhDelta::Skip(1));
                        } else {
                            *len += 1;
                        }
                    }
                    _ => deltas.push(BruhDelta::Skip(1)),
                }
            }

            *next = Frame::Delta(deltas);
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut b = Vec::with_capacity(self.frames.len() * self.width * self.height * 4);

        let w_bytes = self.width.to_ne_bytes();
        let h_bytes = self.width.to_ne_bytes();

        b.extend(w_bytes);
        b.extend(h_bytes);

        for fr in &self.frames {
            match fr {
                Frame::Key(img) => {
                    b.extend(b"k");
                    b.extend(img.encode(self.width));
                    b.extend(b"\n");
                }
                Frame::Delta(delt) => {
                    b.extend(b"t");
                    b.extend(BruhDelta::encode(&delt, self.width * self.height));
                }
            }
        }

        b
    }
}
