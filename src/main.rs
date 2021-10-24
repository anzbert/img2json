use std::fs::File;
use std::io::prelude::*;
use std::{env, process};

extern crate image;

use image::GenericImageView;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Output {
    pixels: Vec<Vec<u8>>,
    size_x: u32,
    size_y: u32,
}
impl Output {
    fn new(pixels: Vec<Vec<u8>>, size_x: u32, size_y: u32) -> Output {
        Self {
            pixels,
            size_x,
            size_y,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {}
        _ => {
            println!(
                "\nError: Invalid Parameters\n\n
            Syntax: img2json [image_file]\n"
            );
            process::exit(1);
        }
    }

    let filename = args.get(1).unwrap();

    let img = match image::open(filename) {
        Ok(img) => img,
        Err(err) => {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
        }
    };

    // resizer later??
    // let resized_img = img.resize(8, 8, FilterType::Nearest);

    let output: Vec<Vec<u8>> = img
        .to_bytes()
        .chunks(3)
        .map(|bytes| bytes.to_vec())
        .collect();
    println!("number of bytes: {:?}", output.len());

    let (size_x, size_y) = img.dimensions();
    println!("dimensions {:?}", img.dimensions());

    let output_struct = Output::new(output, size_x, size_y);
    let output_json = serde_json::to_string(&output_struct).unwrap();

    let mut file = File::create("output.json").unwrap();
    file.write_all(output_json.as_bytes()).unwrap();
    println!("\nWritten to: 'output.json'\n");
}
