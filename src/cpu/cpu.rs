use std::mem;

use crate::{cpu::Registers, memory::Memory};

pub const OP_COUNT: usize = 0xff;
pub const OP_CB_COUNT: usize = 0xff;

pub struct CPU {
  pub mem: Memory,
  pub reg: Registers,

  opcode: u8,
}

impl CPU {
  pub fn new() -> CPU {
    let reg = Registers::new();
    let mem = Memory::new();

    CPU {
      reg,
      mem,
      opcode: 0,
    }
  }

  pub fn run_cycle(&mut self) {
    self.opcode = self.fetch_opcode()
  }

  pub fn fetch_opcode(&self) -> u8 {
    self.mem.read(self.reg.pc)
  }

  pub fn execute_opcode(&mut self) {
    if self.opcode == 0xcb {
      self.opcode = self.fetch_opcode();
      return self._call_operation_cb();
    }
    self._call_operation();
  }

  fn _call_operation(&mut self) {
    let ref mut r = self.reg;
    let ref mut m = self.mem;
    match self.opcode {
      // NOP
      0x00 => {}
      // 8-Bit Loads
      // Load into A
      0x0a => r.a = m.read(r.get_bc() as usize),
      0x1a => r.a = m.read(r.get_de() as usize),
      0x2a => r.a = m.read(r.get_hl_inc() as usize),
      0x3a => r.a = m.read(r.get_hl_dec() as usize),
      0x3e => r.a = r.get_n(m),
      0x78 => r.a = r.b,
      0x79 => r.a = r.c,
      0x7a => r.a = r.d,
      0x7b => r.a = r.e,
      0x7c => r.a = r.h,
      0x7d => r.a = r.l,
      0x7e => r.a = m.read(r.get_hl() as usize),
      0x7f => {}
      // Load into B
      0x06 => r.b = r.get_n(m),
      0x40 => {}
      0x41 => r.b = r.c,
      0x42 => r.b = r.d,
      0x43 => r.b = r.e,
      0x44 => r.b = r.h,
      0x45 => r.b = r.l,
      0x46 => r.b = m.read(r.get_hl() as usize),
      0x47 => r.b = r.a,
      // Load into C
      0x0e => r.c = r.get_n(m),
      0x48 => r.c = r.b,
      0x49 => {}
      0x4a => r.c = r.d,
      0x4b => r.c = r.e,
      0x4c => r.c = r.h,
      0x4d => r.c = r.l,
      0x4e => r.c = m.read(r.get_hl() as usize),
      0x4f => r.c = r.a,
      // Load into D
      0x16 => r.d = r.get_n(m),
      0x50 => r.d = r.b,
      0x51 => r.d = r.c,
      0x52 => {}
      0x53 => r.d = r.e,
      0x54 => r.d = r.h,
      0x55 => r.d = r.l,
      0x56 => r.d = m.read(r.get_hl() as usize),
      0x57 => r.d = r.a,
      // Load into E
      0x1e => r.e = r.get_n(m),
      0x58 => r.e = r.b,
      0x59 => r.e = r.c,
      0x5a => r.e = r.d,
      0x5b => {}
      0x5c => r.e = r.h,
      0x5d => r.e = r.l,
      0x5e => r.e = m.read(r.get_hl() as usize),
      0x5f => r.e = r.a,
      // Load into H
      0x26 => r.h = r.get_n(m),
      0x60 => r.h = r.b,
      0x61 => r.h = r.c,
      0x62 => r.h = r.d,
      0x63 => r.h = r.e,
      0x64 => {}
      0x65 => r.h = r.l,
      0x66 => r.h = m.read(r.get_hl() as usize),
      0x67 => r.h = r.a,
      // Load into L
      0x2e => r.l = r.get_n(m),
      0x68 => r.l = r.b,
      0x69 => r.l = r.c,
      0x6a => r.l = r.d,
      0x6b => r.l = r.e,
      0x6c => r.l = r.h,
      0x6d => {}
      0x6e => r.l = m.read(r.get_hl() as usize),
      0x6f => r.l = r.a,
      // Load into (r16)
      0x02 => m.write(r.get_bc() as usize, r.a),
      0x12 => m.write(r.get_de() as usize, r.a),
      0x22 => m.write(r.get_hl_inc() as usize, r.a),
      0x32 => m.write(r.get_hl_dec() as usize, r.a),
      0x36 => m.write(r.get_hl() as usize, r.get_n(m)),
      0x70 => m.write(r.get_hl() as usize, r.b),
      0x71 => m.write(r.get_hl() as usize, r.c),
      0x72 => m.write(r.get_hl() as usize, r.d),
      0x73 => m.write(r.get_hl() as usize, r.e),
      0x74 => m.write(r.get_hl() as usize, r.h),
      0x75 => m.write(r.get_hl() as usize, r.l),
      0x77 => m.write(r.get_hl() as usize, r.a),
      0xe0 => m.write((r.get_n(m) as usize) + 0xff00, r.a),
      0xe1 => m.write((r.c as usize) + 0xff00, r.a),
      0xea => m.write(r.get_nn(m) as usize, r.a),
      // 16-Bit Loads
      // Load into r16 or SP
      0x01 | 0x11 | 0x21 | 0x31 => {
        let nn = r.get_nn(m);

        match self.opcode {
          0x01 => r.set_bc(nn),
          0x11 => r.set_de(nn),
          0x21 => r.set_hl(nn),
          0x31 => r.sp = nn as usize,
          _ => {}
        }
      }
      0xf9 => r.sp = r.get_hl() as usize,
      // Load into nn
      0x08 => m.write_u16(m.read(r.get_nn(m) as usize) as usize, r.sp as u16),
      _ => {}
    }
  }

  fn _call_operation_cb(&self) {}

  fn _alu_add() {}
  fn _alu_adc() {}
  fn _alu_sub() {}
}
