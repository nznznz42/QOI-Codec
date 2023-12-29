use crate::colour::Colour;

pub fn write32(bytes: &mut Vec<u8>, index: &mut usize, value: u32) {
    bytes.push(((0xff000000 & value) >> 24) as u8);
    bytes.push(((0x00ff0000 & value) >> 16) as u8);
    bytes.push(((0x0000ff00 & value) >> 8) as u8);
    bytes.push(((0x000000ff & value) >> 0) as u8);
    *index += 4;
}

pub fn hash(colour: Colour) -> usize {
    let r: u32 = colour.r as u32;
    let g: u32 = colour.g as u32;
    let b: u32 = colour.b as u32;
    let a: u32 = colour.a as u32;

    return ((r * 3 + g * 5 + b * 7 + a * 11) %  64) as usize
}