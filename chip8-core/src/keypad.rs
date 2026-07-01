pub struct Keypad {
    pub keys: [bool; 16],
}

#[allow(dead_code)]
impl Keypad {
    pub fn new() -> Self {
        Keypad { keys: [false; 16] }
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    pub fn first_pressed_key(&self) -> Option<u8> {
        self.keys
            .iter()
            .position(|&pressed| pressed)
            .map(|index| index as u8)
    }

    pub fn set_pressed(&mut self, key: u8, pressed: bool) {
        self.keys[key as usize] = pressed;
    }
}
