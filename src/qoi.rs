use crate::colour::Colour;
use crate::consts::{QOI_END_MARKER, QOI_END_MARKER_SIZE, QOI_HEADER_SIZE, QOI_OP_DIFF, QOI_OP_INDEX, QOI_OP_LUMA, QOI_OP_RGB, QOI_OP_RGBA, QOI_OP_RUN};
use crate::utils::{hash, write32};

pub struct QoiImage {
    magic: &'static str, //the string qoif in bytes
    height: u32, //height in pixels (Big Endian)
    width: u32, //width in pixels (Big Endian)
    channels: u8, // 3 = RGB, 4 = RGBA
    colourspace: u8, // 0 = sRGB with linear alpha, 1 = all channels linear (not useful but make sure to write it in, set to 1 by default)
}

impl QoiImage {
    pub fn encode(data: Vec<u8>, height: u32, width: u32, channels: u8) -> Vec<u8> {
        let image_size = data.len();
        let last_pixel = image_size - channels as usize;

        let mut prev_colour: Colour = Colour {
            r: 0,
            g: 0,
            b: 0,
            a: 255
        };

        let mut run = 0;
        let mut seen_pixels: [Colour; 64] = [Colour { r: 0, g: 0, b: 0, a: 0 }; 64];

        let max_size = height * width + (channels + 1) as u32 + QOI_HEADER_SIZE as u32 + QOI_END_MARKER_SIZE as u32;
        let mut bytes = Vec::with_capacity(max_size as usize);
        let mut index = 0usize;

        write32(&mut bytes, &mut index, 0x716f6966);
        write32(&mut bytes, &mut index, width);
        write32(&mut bytes, &mut index, height);

        bytes.insert(index, channels.to_be());
        index += 1;
        bytes.insert(index, 1);
        index += 1;

        let mut cur_pixel = Colour {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        };

        for offset in (0..last_pixel).step_by(channels as usize) {
            println!("{}", offset);
            cur_pixel.set_r(data[offset + 0]);
            cur_pixel.set_g(data[offset + 1]);
            cur_pixel.set_b(data[offset + 2]);

            if channels == 4 {
                cur_pixel.set_a(data[offset + 3]);
            } else {
                cur_pixel.set_a(prev_colour.a);
            }

            if cur_pixel == prev_colour {
                run += 1;
                if run == 62 || offset == last_pixel {
                    bytes.insert(index, QOI_OP_RUN | (run - 1));
                    index += 1;
                    run = 0;
                }
            } else {
                if run > 0 {
                    bytes.insert(index, QOI_OP_RUN | (run - 1));
                    index += 1;
                    run = 0;
                }

                let hash = hash(cur_pixel);

                if cur_pixel == seen_pixels[hash] {
                    bytes.insert(index, QOI_OP_INDEX | (hash as u8));
                    index += 1;
                } else {
                    seen_pixels[hash] = cur_pixel.clone();

                    let diff = Colour::diff(cur_pixel, prev_colour);
                    let dr_dg = diff.r.wrapping_sub(diff.g);
                    let db_dg = diff.b.wrapping_sub(diff.g);

                    if diff.a == 0 {
                        if (diff.r as i8 >= -2 && diff.r <= 1) && (diff.g as i8 >= -2 && diff.g <= 1) && (diff.b as i8 >= -2 && diff.b <= 1) {
                            bytes.insert(
                                index,
                                QOI_OP_DIFF
                                    | ((diff.r + 2) << 4)
                                    | ((diff.g + 2) << 2)
                                    | ((diff.b + 2) << 0)
                            );
                            index += 1;
                        } else if (diff.g as i8 >= -32 && diff.g <= 31) && (dr_dg as i8 >= -8 && dr_dg <= 7) && (db_dg as i8 >= -8 && db_dg <= 7) {
                            bytes.insert(index, QOI_OP_LUMA | (diff.g + 32));
                            index += 1;
                            bytes.insert(index, (((dr_dg + 8) << 4) | (db_dg + 8)).try_into().unwrap());
                            index += 1;
                        } else {
                            bytes.insert(index, QOI_OP_RGB);
                            index += 1;
                            bytes.insert(index, cur_pixel.r);
                            index += 1;
                            bytes.insert(index, cur_pixel.g);
                            index += 1;
                            bytes.insert(index, cur_pixel.b);
                            index += 1;
                        }
                    } else {
                        bytes.insert(index, QOI_OP_RGBA);
                        index += 1;
                        bytes.insert(index, cur_pixel.r);
                        index += 1;
                        bytes.insert(index, cur_pixel.g);
                        index += 1;
                        bytes.insert(index, cur_pixel.b);
                        index += 1;
                        bytes.insert(index, cur_pixel.a);
                        index += 1;
                    }
                }
            }
            prev_colour = cur_pixel.clone();
        }
        for byte in QOI_END_MARKER {
            bytes.insert(index, byte);
            index += 1;
        }
        return bytes;
    }
}


