pub const FPS: u8 = 60;
pub const CLOCK_SPEED: u32 = 4194304;
pub const MAX_CYCLES: u16 = CLOCK_SPEED / FPS;

struct GameBoy {
  processor: Processor,
  cycles_this_frame: u16,
}

impl GameBoy {
  fn new() -> GameBoy {
    GameBoy { cycles: 0 }
  }

  fn update(&self) {
    while self.cycles_this_frame < MAX_CYCLES {
      // let cycles = self.cpu.execute_opcode();
      // self.cycles_this_frame += cycles;
      // self.update_timers();
      // self.update_graphics();
      // self.handle_interrupts();
    }
  }
}
