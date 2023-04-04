use usb_play::g213_keyboard::{find_g213_keyboard, set_whole_keyboard_color};

use std::env::args;

fn main() {
    let args: Vec<_> = args().skip(1).collect();

    let mut color: u32 = 0xffd0c0;

    if args.len() >= 1 {
        color = u32::from_str_radix(&args[0], 16).unwrap_or(0xff0000);
    }

    if let Some(device) = find_g213_keyboard() {
        set_whole_keyboard_color(device, color);
    }
}
