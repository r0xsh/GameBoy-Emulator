pub enum GpuMode {
    HBLANK,
    VBLANK,
    OAM,
    VRAM,
}

pub struct Ppu {
    pub mode: GpuMode,
    pub tick: u64,
    pub scanline: u8,
    pub control: u8,
    pub scroll_x: u8,
    pub scroll_y: u8,
    pub ly_compare: u8
}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            mode: GpuMode::HBLANK,
            tick: 0,
            scanline: 0,
            control: 0,
            scroll_x: 0,
            scroll_y: 0,
            ly_compare: 0
        }
    }
}