use crate::util;

pub const MEMORY_SIZE: usize = 0x10000;

pub struct Memory {
  memory: [u8; MEMORY_SIZE],
}

impl Memory {
  pub fn new() -> Memory {
    let memory = [0; MEMORY_SIZE];

    Memory { memory }
  }

  pub fn read(&self, addr: usize) -> u8 {
    self.memory[addr]
  }
  pub fn read_u16(&self, addr: usize) -> u16 {
    util::bit::merge_u8(self.read(addr), self.read(addr + 1))
  }

  pub fn write(&mut self, addr: usize, value: u8) {
    self.memory[addr] = value;
  }

  pub fn write_u16(&mut self, addr: usize, value: u16) {
    let (lo, hi) = util::bit::split_u16(value);
    self.memory[addr] = hi;
    self.memory[addr + 1] = lo;
  }
}
