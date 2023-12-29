pub const MAGIC: &str = "qoif";
pub const QOI_HEADER_SIZE: u8 = 0xe;
pub const QOI_END_MARKER: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 1];
pub const QOI_END_MARKER_SIZE: u8 = QOI_END_MARKER.len() as u8;
pub const QOI_OP_INDEX: u8 = 0x00;
pub const QOI_OP_DIFF: u8 = 0x40;
pub const QOI_OP_LUMA: u8 = 0x80;
pub const QOI_OP_RUN: u8 = 0xc0;
pub const QOI_OP_RGB: u8 = 0xfe;
pub const QOI_OP_RGBA: u8 = 0xff;
pub const QOI_MASK_2: u8 = 0xc0;