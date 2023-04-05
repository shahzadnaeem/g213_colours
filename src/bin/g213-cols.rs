use g213_colours::g213_keyboard::find_g213_keyboard;
use rusb::{Device, GlobalContext};

use std::{env::args, process::ExitCode};

use g213_colours::commands::{breathe_command, colour_command, cycle_command, help_command};

enum Command {
    Colour(Vec<String>),
    Breathe(Vec<String>),
    Cycle(Vec<String>),
    Help(Vec<String>),
}

fn get_command(args: Vec<String>) -> Command {
    let cmd = if args.is_empty() { "" } else { &args[0] };

    match cmd.to_lowercase().as_str() {
        "colour" => Command::Colour(args.into_iter().skip(1).collect()),
        "breathe" => Command::Breathe(args.into_iter().skip(1).collect()),
        "cycle" => Command::Cycle(args.into_iter().skip(1).collect()),
        "help" => Command::Help(args.into_iter().skip(1).collect()),
        _ => Command::Colour(args),
    }
}

trait Run {
    fn run(&self, device: Device<GlobalContext>) -> ExitCode;
}

impl Run for Command {
    fn run(&self, device: Device<GlobalContext>) -> ExitCode {
        match self {
            Command::Colour(args) => colour_command(device, args),
            Command::Breathe(args) => breathe_command(device, args),
            Command::Cycle(args) => cycle_command(device, args),
            Command::Help(args) => help_command(device, args),
        }
    }
}

fn main() -> ExitCode {
    let device = find_g213_keyboard().expect("No G213 keyboard found, sorry!");

    let args = args().skip(1).collect::<Vec<_>>();

    let command = get_command(args);

    command.run(device)
}
