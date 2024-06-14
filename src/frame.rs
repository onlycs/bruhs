use colors_transform::{AlphaColor, Color, Rgb};
use std::{io, path::PathBuf};

use crate::img::Bruh;

#[derive(Debug)]
pub enum BruhDelta {
    Keep(u32), // keep: length
    Overwrite(Rgb),
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
    width: u32,
    height: u32,
    pxperframe: usize,
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
    pub fn parse_gif(path: PathBuf, width: u32, height: u32) -> Result<Self, io::Error> {
        let mut pngsdir = path.clone();
        pngsdir.set_extension("pngs_tmp");

        //create dir
        std::fs::create_dir(&pngsdir)?;

        // run ffmpeg
        let output = std::process::Command::new("ffmpeg")
            .args(&[
                "-i",
                path.to_str().unwrap(),
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

    pub fn parse_dir(path: PathBuf, width: u32, height: u32) -> Result<Self, std::io::Error> {
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
            pxperframe: (width * height) as usize,
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
            if diff > (self.pxperframe / 10) * 9 {
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
                    Some(BruhDelta::Keep(len)) => {
                        if i % self.width as usize == 0 {
                            deltas.push(BruhDelta::Keep(1));
                        } else {
                            *len += 1;
                        }
                    }
                    _ => deltas.push(BruhDelta::Keep(1)),
                }
            }

            *next = Frame::Delta(deltas);
        }
    }
}
