use usb_play::g213_keyboard::{find_g213_keyboard, set_whole_keyboard_color};

use std::env::args;

fn main() {
    let args: Vec<_> = args().skip(1).collect();

    const WHITE: u32 = 0xffd0c0;
    const RED: u32 = 0xff1010;
    const GREEN: u32 = 0x10f010;

    let mut color: u32 = WHITE;

    if args.len() >= 1 {
        color = u32::from_str_radix(&args[0], 16).unwrap_or(RED);
    }

    if let Some(device) = find_g213_keyboard() {
        set_whole_keyboard_color(device, color);
    }
}
