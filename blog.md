# Making a Rust Tiling Window Manager (rwm)

### What

This will be a (probably worse) clone of [dwm](https://dwm.suckless.org) written in
[rust](https://www.rust-lang.org) using [xcb](https://crates.io/crates/xcb) (instead of xlib).

### Why?

-   Learning experience
-   Hacking in C (for dwm) I don't like
-   Hacking in Rust I like

## Minimal "Window Manager"

Well then let's just get started.
The goal for now is to have a windows manager, that I can spawn windows in (like my terminal).

### Setting up the project

A simple

```sh
cargo new rwm
```

does the trick. I'll add needed dependencies later.

### Logging

Very important for this kind of development.
Testing a window manager and it just crashing on you without knowing what happened is a painful experience.
With logging you at least get an idea of what went wrong.

I've had great experience with [env_logger](https://crates.io/crates/env_logger) in the past but for this project
I wanted to try out [tracing](https://crates.io/crates/tracing) with [tracing-subscriber](https://crates.io/crates/tracing-subscriber).
So let's just add these as dependencies.

```sh
cargo add tracing
cargo add tracing-subscriber
```

And set up logging

```rust
// src/main.rs

#[macro_use]
extern crate tracing;

use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    util::SubscriberInitExt,
};

fn main() {
    fmt::fmt()
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .finish()
        .init();

    info!("Starting rwm!");
}
```

### Connecting to X

To connect to x we will need some bindings to communicate with the server. For that I'm using [xcb](https://crates.io/crates/xcb).

```sh
cargo add xcb
```

I want to have a struct, that keeps all the state (including the connection).

```rust
// src/rwm.rs

pub struct Rwm {
    connection: xcb::Connection,
}

impl Rwm {
    pub fn new() -> Self {
        let (connection, _) = xcb::Connection::connect(None).unwrap();

        Self { connection }
    }

    pub fn setup(&mut self) {}

    pub fn run(&mut self) {}
}
```

```rust
// src/main.rs

//...

mod rwm;

fn main() {
    // ...

    let mut rwm = rwm::Rwm::new();
    rwm.setup();
    rwm.run();
}
```

Great. Now we connected to X but we gotta do at least something to see, if it's working.
Let's get the [root window](https://en.wikipedia.org/wiki/Root_window)
(basically X just spawn a window the size of your screen, that's the parent of all other windows)
and have it report all key presses to us. We will need to change the EventMask of the root window
and report every event.

```rust
// src/rwm.rs

use xcb::x

struct Rwm {
    // ...
    root: x::Window,
}

impl Rwm {
    pub fn new() -> Self {
        // ...

        let setup = connection.get_setup();
        let screen = setup.roots().next.unwrap();
        let root = screen.root();

        Self { connection, root }
    }

    pub fn setup(&mut self) {
        self.connection.send_request(&x::ChangeWindowAttributes {
            window: self.root,
            value_list: &[x::Cw::EventMask(x::EventMask::KEY_PRESS)],
        });

        self.connection.flush().unwrap();
    }

    pub fn run(&mut self) {
        loop {
            if let Ok(event) = self.connection.wait_for_event() {
                info!(?event);
            }

            self.connection.flush().unwrap();
        }
    }
}
```

Now we actually want to debug this. For that I'm using [Xephyr](https://wiki.archlinux.org/title/Xephyr) and [just](https://github.com/casey/just).

```sh
* justfile

run:
    Xephyr -ac -br -noreset -screen 800x600 :1 &> /dev/null &
    DISPLAY=:1 cargo run

clean:
    cargo clean
```

Not if we `just run` xephyr will open a window which will capture our key presses and output them.
Let's look at a such an output when pressing Q.

```sh
... event=X(KeyPress(KeyPressEvent { detail: 24, ... }))
```

The `detail` field will tells us, which key was pressed but hmm..
24 is neither the ASCII value for Q nor anything related.
So what happened here? If we take a look at the [documentation](https://rust-x-bindings.github.io/rust-xcb/xcb/x/struct.KeyPressEvent.html*method.detail)
we will get our answer. This number is just a keycode representing the key we pressed.
We need some kind of mapping to recognize, which key was pressed.

## Keymap

At this point I wanted to use [xkbcommon](https://crates.io/crates/xkbcommon)
but to be honestly, it's just a mess (probably not their fault but xserver).

So I quickly whipped up my own keycode to keysym mapper.

First I "borrowed" all the keysmys from [xkb/keysyms](https://github.com/rust-x-bindings/xkbcommon-rs/blob/master/src/xkb/keysyms.rs)
(I did replace every `: u32` with `: x::Keysym` though).

```rust
// src/keysyms.rs

// ...

*![allow(non_upper_case_globals, dead_code)]

use xcb::x;

/// Special ``KeySym``
pub const KEY_NoSymbol: x::Keysym = 0x0000_0000;

// ...
```

Next this simple code translates from keycode to keysym and the other way.

```rust
// src/keymap.rs

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

    pub fn keysym(&self, keycode: u8) -> u32 {
        self.keysyms[(keycode - self.min_keycode) as usize * self.keysyms_per_keycode as usize]
    }

    pub fn keycode(&self, keysym: u32) -> u8 {
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
```

Now to create that keymap.

```rust
// src/rwm.rs

// ...

use crate::keymap::Keymap;

pub struct Rwm {
    // ...
    keymap: Keymap,
}

impl Rwm {
    pub fn new() -> Self {
        // ...

        let keyboard_mapping_cookie = connection.send_request(&x::GetKeyboardMapping {
            first_keycode: setup.min_keycode(),
            count: setup.max_keycode() - setup.min_keycode() + 1,
        });

        let keyboard_mapping = connection.wait_for_reply(keyboard_mapping_cookie).unwrap();

        let keymap = Keymap::new(
            keyboard_mapping.keysyms().to_vec(),
            setup.min_keycode(),
            keyboard_mapping.keysyms_per_keycode(),
        );

        connection.flush().unwrap();

        Self {
            connection,
            root,
            keymap,
        }
    }

    // ...
}
```

Let's log the keypresses with the right keymap.

```rust
// ...

impl Rwm {
    // ...

    pub fn run(&mut self) {
        loop {
            if let Ok(event) = self.connection.wait_for_event() {
                match event {
                    xcb::Event::X(x::Event::KeyPress(event)) => self.key_press(event),
                    _ => {}
                }
            }

            self.connection.flush().unwrap();
        }
    }

    fn key_press(&mut self, event: x::KeyPressEvent) {
        let keysym = self.keymap.keysym(event.detail());
        let state = event.state();

        info!(keysym, ?state);
    }
}
```

Finally I'm getting `113` for Q, which is the correct ASCII value!

## Spawning programs

Well it would be nice, if our keypresses actually mapped to programs. Let's implement that.
We will need some kind of struct to represent a `Keycombo`

```rust
// src/combo.rs

use xcb::x;

*[derive(Eq, PartialEq, Hash)]
pub struct Key {
    mask: x::KeyButMask,
    key: x::Keysym,
}

impl Key {
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
```

Let's add a `Key` to `fn(&mut Rwm)` map and a `spawn` method to spawn programs.
Let's also grab the `Key`s in the `setup` method.

```rust
// src/rwm.rs

use std::{collections::HashMap, process::Command};

use crate::{combo::Key, keymap::Keymap};

// ...

pub struct Rwm {
    // ...
    keys: HashMap<Key, fn(&mut Self)>,
}


impl Rwm {
    pub fn new() -> Self {
        // ...

        Self {
            // ...
            keys: HashMap::new(),
        }
    }

    pub fn setup(&mut self) {
        // ...

        for key_combo in self.keys.keys() {
            self.connection.send_request(&x::GrabKey {
                owner_events: true,
                grab_window: self.root,
                modifiers: x::ModMask::from_bits_truncate(key_combo.mask().bits()),
                key: self.keymap.keycode(key_combo.key()),
                pointer_mode: x::GrabMode::Async,
                keyboard_mode: x::GrabMode::Async,
            });
        }

        // ...
    }

    // ...

    pub fn spawn(&mut self, command: &str, args: &[&str]) {
        Command::new(command).args(args).spawn().unwrap();
    }
}
```

Add some useful macros to make it easy to create map `Key` to `fn(&mut Rwm)`.

```rust
// src/macros.rs

macro_rules! spawn {
    ( $command:expr ) => {
        |rwm: &mut $crate::rwm::Rwm| rwm.spawn($command, &[])
    };
    ( $command:expr, $( $arg:expr ),* $( , )? ) => {
        |rwm: &mut $crate::rwm::Rwm| rwm.spawn($command, &[$($arg,)*])
    };
}

macro_rules! keys {
    ( $( $tup:expr ),* $( , )? ) => {
        pub const KEYS: [($crate::combo::Key, fn(&mut $crate::rwm::Rwm)); count!($($tup)*)] = [$(($crate::combo::Key::new($tup.0, $tup.1), $tup.2)),*];
    };
}

macro_rules! count {
    () => (0);
    ( $x:tt $($xs:tt)* ) => (1 + count!($($xs)*));
}
```

```rust
// main.rs

// ...

#[macro_use]
mod macros;

// ...
```

And finally a config file, where we add all the `Key`s.

```rust
// src/config.rs

use xcb::x;

use crate::keysyms::*;

const MOD: x::KeyButMask = x::KeyButMask::MOD4;
const MODSHIFT: x::KeyButMask = MOD.union(x::KeyButMask::SHIFT);

keys!(
    (MODSHIFT, KEY_Return, spawn!("alacritty")),
    (MOD, KEY_p, spawn!("dmenu_run")),
);
```

```rust
// src/rwm.rs

// ...

use crate::{combo::Key, config, keymap::Keymap};

// ...

impl Rwm {
    pub fn new() -> Self {
        // ...

        Self {
            // ...
            keys: HashMap::from(config::KEYS),
        }
    }
}
```
