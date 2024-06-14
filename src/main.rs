#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{fs::OpenOptions, io::Write};

use eframe::egui;
use egui_extras::RetainedImage;

mod frame;
mod img;

fn main() {
    let bruhs = frame::Bruhs::parse_gif("test.gif".into(), 265, 199).unwrap();
    let encode = bruhs.encode();

    let mut open = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("test.bruhs")
        .unwrap();

    open.write_all(&encode).unwrap();
}
