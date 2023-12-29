use std::fs::File;
use std::io::{Read, Write};

mod qoi;
mod consts;
mod colour;
mod utils;

fn main() {
    let mut file = File::open("../assets/image_assets/monument.bin").unwrap();
    let mut buffer = Vec::new();

    let _f = file.read_to_end(&mut buffer).unwrap();

    let qoibuf = qoi::QoiImage::encode(buffer, 588, 735,4);

    let mut newfile = File::create("./testimg1.qoi").unwrap();
    let _ = newfile.write(&*qoibuf);
}
