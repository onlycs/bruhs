#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui_extras::RetainedImage;

mod frame;
mod img;

fn main() {
    let frame = frame::Bruhs::parse_gif("test.gif".into(), 265, 199).unwrap();

    println!("{:?}", frame.frames[4]);
}
