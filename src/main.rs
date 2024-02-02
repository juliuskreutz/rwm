use rwm::Rwm;

#[macro_use]
mod macros;
mod bar;
mod client;
mod combo;
mod config;
mod cursor;
mod draw;
mod key;
mod keymap;
mod monitor;
mod rwm;

fn main() {
    let mut rwm = Rwm::new();
    rwm.setup();
    rwm.run();
}
