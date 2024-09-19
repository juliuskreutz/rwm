use crate::key::*;
use xcb::x;

pub const FONT: &str = "ComicCodeLigatures Nerd Font Mono 24";

tags!("", "", "", "", "", "", "", "", "");

pub const WINDOW_MARGIN: u16 = 10;

pub const WINDOW_BORDER_WIDTH: u16 = 2;
pub const WINDOW_BORDER_COLOR: u32 = 0xcad3f5;
pub const WINDOW_BORDER_HL_COLOR: u32 = 0x8aadf4;

pub const BAR_HEIGHT: u16 = 32;
pub const BAR_TEXT_PADDING: u16 = 12;

pub const BAR_COLOR: u32 = 0x24273a;
pub const BAR_HL_COLOR: u32 = 0x8aadf4;
pub const BAR_TEXT_COLOR: u32 = 0xcad3f5;
pub const BAR_TEXT_HL_COLOR: u32 = 0xffffff;

const MOD: x::KeyButMask = x::KeyButMask::MOD4;
const MODSHIFT: x::KeyButMask = MOD.union(x::KeyButMask::SHIFT);

const BUTTON1: x::Button = 1;
const BUTTON3: x::Button = 3;

keys!(
    (MODSHIFT, KEY_Return, spawn!("kitty")),
    (MOD, KEY_p, spawn!("rmenu_run")),
    (MOD, KEY_k, spawn!("rmenu_pass")),
    (MOD, KEY_e, spawn!("chromium")),
    (MOD, KEY_s, spawn!("shot")),
    (MOD, KEY_q, spawn!("copyq", "toggle")),
    (MODSHIFT, KEY_c, kill!()),
    (MOD, KEY_Return, swap!()),
    (MOD, KEY_f, toggle_fullscreen!()),
    (MOD, KEY_space, toggle_floating!()),
    (MOD, KEY_Left, main_factor!(-0.05)),
    (MOD, KEY_Right, main_factor!(0.05)),
    (MOD, KEY_1, view!(0)),
    (MOD, KEY_2, view!(1)),
    (MOD, KEY_3, view!(2)),
    (MOD, KEY_4, view!(3)),
    (MOD, KEY_5, view!(4)),
    (MOD, KEY_6, view!(5)),
    (MOD, KEY_7, view!(6)),
    (MOD, KEY_8, view!(7)),
    (MOD, KEY_9, view!(8)),
    (MOD, KEY_Up, move_up!()),
    (MOD, KEY_Down, move_down!()),
    (MODSHIFT, KEY_1, tag!(0)),
    (MODSHIFT, KEY_2, tag!(1)),
    (MODSHIFT, KEY_3, tag!(2)),
    (MODSHIFT, KEY_4, tag!(3)),
    (MODSHIFT, KEY_5, tag!(4)),
    (MODSHIFT, KEY_6, tag!(5)),
    (MODSHIFT, KEY_7, tag!(6)),
    (MODSHIFT, KEY_8, tag!(7)),
    (MODSHIFT, KEY_9, tag!(8)),
    (MODSHIFT, KEY_period, tagmon!()),
    (MODSHIFT, KEY_q, quit!()),
);

buttons!((MOD, BUTTON1, drag!()), (MOD, BUTTON3, resize!()));
