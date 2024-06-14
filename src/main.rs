#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{
    env,
    fs::{self, File},
    path::PathBuf,
};

use bruhs::Bruhs;
use itertools::Itertools;

mod bruhs;
mod img;

fn main() {
    let args = env::args();
    let args = args.collect_vec();

    match args[1].as_str() {
        "compile" => {
            let gifpath = PathBuf::from(&args[2]);
            let bruhspath = {
                let mut tmp = gifpath.clone();
                tmp.set_extension("bruhs");
                tmp
            };

            let mut gif = gif::Decoder::new(File::open(&gifpath).unwrap()).unwrap();
            let frameone = gif.read_next_frame().unwrap().unwrap();
            let width = frameone.width;
            let height = frameone.height;

            drop(gif);

            let bruhs = Bruhs::parse_gif(gifpath, width as usize, height as usize).unwrap();
            let encode = bruhs.encode();

            fs::write(bruhspath, encode).unwrap();
        }
        "decompile" => {
            let bruhspath = PathBuf::from(&args[2]);
            let gifpath = {
                let mut tmp = bruhspath.clone();
                tmp.set_extension("gif");
                tmp
            };

            let bruhs = Bruhs::decode(fs::read(bruhspath).unwrap());
            bruhs.into_gif(&gifpath).unwrap();
        }
        _ => {
            let bruhspath = PathBuf::from(&args[1]);
            let gifpath = PathBuf::from(".tmp.gif");

            // remove gifpath if exists
            fs::remove_file(&gifpath).unwrap_or_default();

            let bruhs = Bruhs::decode(fs::read(bruhspath).unwrap());
            bruhs.into_gif(&gifpath).unwrap();

            open::that_detached(gifpath).unwrap();
        }
    }
}
