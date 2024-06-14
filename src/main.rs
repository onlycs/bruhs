#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{
    fs::OpenOptions,
    io::{Read, Write},
};

use eframe::egui;
use egui_extras::RetainedImage;
use itertools::Itertools;

mod frame;
mod img;

fn main() {
    let bruhs = frame::Bruhs::parse_gif("test.gif".into(), 265, 199).unwrap();
    let encode = bruhs.encode();

    let mut open = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("test.bruhs")
        .unwrap();

    open.set_len(0).unwrap();
    open.write_all(&encode).unwrap();
    drop(open);

    let read = OpenOptions::new().read(true).open("test.bruhs").unwrap();
    let bytes = read
        .bytes()
        .into_iter()
        .filter_map(Result::ok)
        .collect_vec();

    println!("{}", bytes == encode);

    let decode = frame::Bruhs::decode(bytes);

    println!("{}", decode == bruhs);
}
