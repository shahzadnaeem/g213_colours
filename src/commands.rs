use crate::g213_keyboard::{
    limit_speed, set_breathe, set_cycle, set_keyboard_colour, set_region_colour, KeyboardRegions,
};
use crate::x11_colours::get_x11_colour;
use rusb::{Device, GlobalContext};
use std::process::ExitCode;

pub enum Command<'a> {
    Colour(&'a [String]),
    Region(&'a [String]),
    Breathe(&'a [String]),
    Cycle(&'a [String]),
    Help(&'a [String]),
    Unknown(&'a [String]),
}

pub fn get_command(args: &'_ [String]) -> Command {
    let cmd = if args.is_empty() { "" } else { &args[0] };

    match cmd.to_lowercase().as_str() {
        "colour" => Command::Colour(&args[1..]),
        "region" => Command::Region(&args[1..]),
        "breathe" => Command::Breathe(&args[1..]),
        "cycle" => Command::Cycle(&args[1..]),
        "help" => Command::Help(&args[1..]),
        _ => Command::Unknown(args),
    }
}

pub trait Run {
    fn run(&self, device: Device<GlobalContext>) -> ExitCode;
}

impl Run for Command<'_> {
    fn run(&self, device: Device<GlobalContext>) -> ExitCode {
        match self {
            Command::Colour(args) => colour_command(device, args),
            Command::Region(args) => region_command(device, args),
            Command::Breathe(args) => breathe_command(device, args),
            Command::Cycle(args) => cycle_command(device, args),
            Command::Help(args) => help_command(device, args),
            Command::Unknown(cmd) => {
                eprintln!("Uknown command: '{}'", cmd.join(" "));
                ExitCode::FAILURE
            }
        }
    }
}

// ----------------------------------------------------------------------------

fn get_colour_or_red(args: &[String]) -> (u32, ExitCode) {
    const RED: u32 = 0xff1010;

    match get_x11_colour(args) {
        Some(col) => (col, ExitCode::SUCCESS),
        None => (RED, ExitCode::FAILURE),
    }
}

fn colour_command(device: Device<GlobalContext>, args: &[String]) -> ExitCode {
    let (colour, exit_code) = get_colour_or_red(args);

    set_keyboard_colour(device, colour);

    exit_code
}

fn region_command(device: Device<GlobalContext>, args: &[String]) -> ExitCode {
    let mut exit_code = ExitCode::FAILURE;

    if !args.is_empty() {
        let region: KeyboardRegions = args[0].parse::<u8>().unwrap().into();

        let (colour, status) = get_colour_or_red(&args[1..]);

        set_region_colour(device, region as u8, colour);

        exit_code = status;
    } else {
        eprintln!("At least one - 'region' ['colour'] - argument needed for 'region' command");
    }

    exit_code
}

fn breathe_command(device: Device<GlobalContext>, args: &[String]) -> ExitCode {
    let mut exit_code = ExitCode::FAILURE;

    if !args.is_empty() {
        let speed = limit_speed(args[0].parse::<u16>().unwrap());

        let (colour, status) = get_colour_or_red(&args[1..]);

        set_breathe(device, speed, colour);

        exit_code = status
    } else {
        eprintln!("At least one - 'speed' ['colour'] - argument needed for 'breathe' command");
    }

    exit_code
}

fn cycle_command(device: Device<GlobalContext>, args: &[String]) -> ExitCode {
    let mut exit_code = ExitCode::FAILURE;

    if args.len() == 1 {
        let speed = limit_speed(args[0].parse::<u16>().unwrap());

        set_cycle(device, speed);

        exit_code = ExitCode::SUCCESS;
    } else {
        eprintln!("One 'speed' argument needed for 'cycle' command");
    }

    exit_code
}

fn help_command(_device: Device<GlobalContext>, _args: &[String]) -> ExitCode {
    println!("Help ...");

    // TODO: Give some device details
    println!("You do have a G213 keyboard ✅");

    // TODO: Usual help stuff

    ExitCode::SUCCESS
}
