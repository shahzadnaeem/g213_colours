use usb_play::{
    g213_keyboard::{find_g213_keyboard, set_whole_keyboard_color},
    x11_colours::get_colour_def,
};

use std::env::args;

fn main() {
    let args: Vec<_> = args().skip(1).collect();

    const WHITE: u32 = 0xffd0c0;
    const RED: u32 = 0xff1010;

    let mut color: u32 = WHITE;

    if args.len() >= 1 {
        if let Ok(numeric_col) = u32::from_str_radix(&args[0], 16) {
            color = numeric_col
        } else if let Some(named_col) = get_colour_def(&args[0]) {
            color = named_col
        } else {
            color = RED;
        }

        // println!("Colour {} => {:06x}", &args[0], color);
    }

    if let Some(device) = find_g213_keyboard() {
        set_whole_keyboard_color(device, color);
    }
}
