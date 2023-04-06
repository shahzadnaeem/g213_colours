use g213_colours::g213_keyboard::find_g213_keyboard;

use std::{env::args, process::ExitCode};

use g213_colours::commands::{get_command, Run};

fn main() -> ExitCode {
    let device = find_g213_keyboard().expect("No G213 keyboard found, sorry!");

    let args = args().skip(1).collect::<Vec<_>>();

    let command = get_command(&args);

    command.run(device)
}
