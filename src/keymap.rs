pub struct Keymap {
    keysyms: Vec<u32>,
    min_keycode: u8,
    keysyms_per_keycode: u8,
}

impl Keymap {
    pub fn new(keysyms: Vec<u32>, min_keycode: u8, keysyms_per_keycode: u8) -> Self {
        Self {
            keysyms,
            keysyms_per_keycode,
            min_keycode,
        }
    }

    pub fn get_keysym(&self, keycode: u8) -> u32 {
        self.keysyms[(keycode - self.min_keycode) as usize * self.keysyms_per_keycode as usize]
    }

    pub fn get_keycode(&self, keysym: u32) -> u8 {
        for (i, keysyms) in self
            .keysyms
            .chunks(self.keysyms_per_keycode as usize)
            .enumerate()
        {
            if keysyms.contains(&keysym) {
                return self.min_keycode + i as u8;
            }
        }

        0
    }
}
