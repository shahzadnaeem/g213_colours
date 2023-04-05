use g213_colours::{
    g213_keyboard::{find_g213_keyboard, set_whole_keyboard_colour},
    x11_colours::get_x11_colour,
};

use std::{env::args, process::ExitCode};

fn main() -> ExitCode {
    let args = args().skip(1).collect::<Vec<_>>();

    // Invalid input colour
    const RED: u32 = 0xff1010;

    let (colour, exit_code) = match get_x11_colour(&args) {
        Some(col) => (col, ExitCode::SUCCESS),
        None => (RED, ExitCode::FAILURE),
    };

    let device = find_g213_keyboard().expect("No G213 keyboard found, sorry!");
    set_whole_keyboard_colour(device, colour);

    exit_code
}
