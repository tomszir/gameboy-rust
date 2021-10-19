pub fn set_bit_u8(b: u8, pos: u8) -> u8 {
  b | (b << pos)
}

pub fn clear_bit_u8(b: u8, pos: u8) -> u8 {
  b & !(b << pos)
}

pub fn get_bit_u8(b: u8, pos: u8) -> u8 {
  (b >> pos) & 0x1
}

pub fn merge_u8(b_hi: u8, b_lo: u8) -> u16 {
  ((b_hi as u16) << 8) | b_lo as u16
}

pub fn split_u16(b: u16) -> (u8, u8) {
  ((b >> 8) as u8, b as u8)
}
