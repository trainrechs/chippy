pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

pub struct Display {
    pixels: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    has_changed: bool, // true if changed since last render — lets main loop skip redundant draws
}
#[allow(dead_code)]
impl Display {
    pub fn new() -> Self {
        Display {
            pixels: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            has_changed: true,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn clear(&mut self) {
        self.pixels = [false; DISPLAY_WIDTH * DISPLAY_HEIGHT];
        self.has_changed = true;
    }

    fn index(&self, x: usize, y: usize) -> usize {
        let x = x % DISPLAY_WIDTH;
        let y = y % DISPLAY_HEIGHT;
        y * DISPLAY_WIDTH + x
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.pixels[self.index(x, y)]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, on: bool) {
        let idx = self.index(x, y);
        self.pixels[idx] = on;
        self.has_changed = true;
    }

    pub fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let mut collision = false;

        for (row, &byte) in sprite.iter().enumerate() {
            for bit in 0..8 {
                let pixel_on = (byte & (0x80 >> bit)) != 0;
                if !pixel_on {
                    continue;
                }

                let px = x + bit;
                let py = y + row;
                let idx = self.index(px, py);

                if self.pixels[idx] {
                    collision = true; // erasing a set pixel = collision
                }
                self.pixels[idx] ^= true; // XOR draw
            }
        }

        self.has_changed = true;
        collision
    }

    pub fn is_dirty(&self) -> bool {
        self.has_changed
    }

    pub fn clear_dirty(&mut self) {
        self.has_changed = false;
    }

    pub fn get_buffer(&self) -> &[bool] {
        &self.pixels
    }
}
