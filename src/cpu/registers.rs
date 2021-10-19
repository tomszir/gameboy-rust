use crate::{memory::Memory, util};

pub const FLAG_CARRY: u8 = 0;
pub const FLAG_HCARRY: u8 = 1;
pub const FLAG_SUB: u8 = 2;
pub const FLAG_ZERO: u8 = 3;

pub struct Registers {
  pub a: u8,
  pub f: u8,
  pub b: u8,
  pub c: u8,
  pub d: u8,
  pub e: u8,
  pub h: u8,
  pub l: u8,

  pub sp: usize,
  pub pc: usize,
}

impl Registers {
  pub fn new() -> Registers {
    Registers {
      a: 0,
      f: 0,
      b: 0,
      c: 0,
      d: 0,
      e: 0,
      h: 0,
      l: 0,
      sp: 0,
      pc: 0,
    }
  }

  pub fn get_pc_inc(&mut self, inc: usize) -> usize {
    let pc = self.pc;
    self.pc += inc;
    pc
  }

  pub fn get_n(&mut self, m: &Memory) -> u8 {
    m.read(self.get_pc_inc(1))
  }

  pub fn get_nn(&mut self, m: &Memory) -> u16 {
    m.read_u16(self.get_pc_inc(2))
  }

  pub fn get_af(&self) -> u16 {
    util::bit::merge_u8(self.a, self.f)
  }

  pub fn set_af(&mut self, value: u16) {
    let (a, f) = util::bit::split_u16(value);
    self.a = a;
    self.f = f;
  }

  pub fn get_bc(&self) -> u16 {
    util::bit::merge_u8(self.b, self.c)
  }

  pub fn set_bc(&mut self, value: u16) {
    let (b, c) = util::bit::split_u16(value);
    self.b = b;
    self.c = c;
  }

  pub fn get_de(&self) -> u16 {
    util::bit::merge_u8(self.d, self.e)
  }

  pub fn set_de(&mut self, value: u16) {
    let (d, e) = util::bit::split_u16(value);
    self.d = d;
    self.e = e;
  }

  pub fn get_hl(&self) -> u16 {
    util::bit::merge_u8(self.h, self.l)
  }

  pub fn get_hl_inc(&mut self) -> u16 {
    let hl = self.get_hl();
    self.set_hl(hl + 1);
    hl
  }

  pub fn get_hl_dec(&mut self) -> u16 {
    let hl = self.get_hl();
    self.set_hl(hl - 1);
    hl
  }

  pub fn set_hl(&mut self, value: u16) {
    let (h, l) = util::bit::split_u16(value);
    self.h = h;
    self.l = l;
  }

  pub fn get_flag(&self, flag: u8) -> u8 {
    util::bit::get_bit_u8(self.f, flag)
  }

  pub fn set_flag(&mut self, flag: u8) {
    self.f = util::bit::set_bit_u8(self.f, flag);
  }

  pub fn clear_flag(&mut self, flag: u8) {
    self.f = util::bit::clear_bit_u8(self.f, flag);
  }
}
