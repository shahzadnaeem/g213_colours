use g213_colours::{
    g213_keyboard::{find_g213_keyboard, set_whole_keyboard_color},
    x11_colours::get_colour_def,
};

use std::env::args;

fn get_colour(args: &Vec<String>) -> Option<u32> {
    let mut colour: Option<u32> = None;

    // Default
    const WHITE: u32 = 0xffd0c0;

    if args.len() == 0 {
        colour = Some(WHITE);
    } else if args.len() == 1 {
        if let Ok(numeric_col) = u32::from_str_radix(&args[0], 16) {
            colour = Some(numeric_col);
        } else if let Some(named_col) = get_colour_def(&args[0]) {
            colour = Some(named_col);
        }
    } else if args.len() == 2 {
        let name = format!("{} {}", &args[0], &args[1]);

        if let Some(named_col) = get_colour_def(&name) {
            colour = Some(named_col)
        }
    } else if args.len() == 3 {
        let name = format!("{} {} {}", &args[0], &args[1], &args[2]);

        if let Some(named_col) = get_colour_def(&name) {
            colour = Some(named_col)
        }
    }

    colour
}

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();

    // Invalid input colours
    const RED: u32 = 0xff1010;

    let colour = get_colour(&args).unwrap_or(RED);

    if let Some(device) = find_g213_keyboard() {
        set_whole_keyboard_color(device, colour);
    }
}
