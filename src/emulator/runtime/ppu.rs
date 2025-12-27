use crate::emulator::runtime::bus::Bus;

pub const WIDTH: usize = 160;
pub const HEIGHT: usize = 144;
pub const BPP: usize = 4;

// white, light gray, dark gray, black
pub const BG_PALETTE: [u32; 4] = [0xE0F8D0FF, 0x88C070FF, 0x346856FF, 0x081820FF];

pub const TILE_MAP_0: u16 = 0x9800;

pub type RGBA8888 = u32;
pub type Tile = Box<[RGBA8888; 8 * 8]>; // fixme: support 8 * 16 tile size
pub type FrameBuffer = Box<[RGBA8888; WIDTH * HEIGHT]>;

pub fn new_buffer() -> FrameBuffer {
    Box::new([0u32; WIDTH * HEIGHT])
}

pub fn render_frame(bus: &mut Bus) {
    // todo: respect priorities
    // render background
    let _background = render_background(bus);
    // render window
    // render objects
    // blit layers together
}

pub fn render_background(bus: &mut Bus) -> FrameBuffer {
    let buffer = new_buffer();

    for y in 0..HEIGHT as u16 - 1 {
        for x in 0..WIDTH as u16 - 1 {
            let tile_index = bus.read_byte(TILE_MAP_0 + x + y * 32).unwrap();
            let tile_address = (tile_index as u16) * 16;
            let _tile = read_tile(bus, tile_address);
        }
    }

    buffer
}

fn read_tile(bus: &mut Bus, base_index: u16) -> Tile {
    let mut tile = [0u32; 8 * 8]; // fixme: ideally use RGBA8888 here instead of u32

    for row_index in 0..8 {
        let first = bus.read_byte(base_index + row_index * 2).unwrap();
        let second = bus.read_byte(base_index + row_index * 2 + 1).unwrap();

        for column_index in 0..8 {
            let bit_low = (first >> 7 - column_index) & 0x1;
            let bit_high = (second >> 7 - column_index) & 0x1;
            let color_index = (bit_high << 1) | bit_low;
            tile[(row_index * 8 + column_index) as usize] = BG_PALETTE[color_index as usize];
        }
    }

    Box::new(tile)
}
