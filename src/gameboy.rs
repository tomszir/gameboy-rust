struct GameBoy {}

// https://www.copetti.org/writings/consoles/game-boy/
// CPU: Sharp SM83 @ ~4.19 MHz
// Memory: 8 KB WRAM (Work RAM)
// Graphics: 8 KB VRAM PPU (160x144) 4 shades of gray

struct Registers {
  a: u16,
}
