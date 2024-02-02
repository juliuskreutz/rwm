use xcb::x;

#[derive(Eq, PartialEq, Hash)]
pub struct KeyCombo {
    mask: x::KeyButMask,
    key: x::Keysym,
}

impl KeyCombo {
    pub const fn new(mask: x::KeyButMask, key: x::Keysym) -> Self {
        Self { mask, key }
    }

    pub fn mask(&self) -> x::KeyButMask {
        self.mask
    }

    pub fn key(&self) -> x::Keysym {
        self.key
    }
}

#[derive(Eq, PartialEq, Hash)]
pub struct ButtonCombo {
    mask: x::KeyButMask,
    button: x::Button,
}

impl ButtonCombo {
    pub const fn new(mask: x::KeyButMask, button: x::Button) -> Self {
        Self { mask, button }
    }

    pub fn button(&self) -> x::Button {
        self.button
    }

    pub fn mask(&self) -> x::KeyButMask {
        self.mask
    }
}
