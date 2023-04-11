use rusb::{Device, GlobalContext};
use serde::{Deserialize, Serialize};

use crate::g213_keyboard::{
    limit_speed, set_breathe, set_cycle, set_keyboard_colour, set_region_colour, KeyboardRegions,
};
use crate::x11_colours::get_x11_colour;

#[repr(u8)]
pub enum Status {
    Success = 0,
    Failure,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Colour(Vec<String>),
    Region(Vec<String>),
    Breathe(Vec<String>),
    Cycle(Vec<String>),
    Help(Vec<String>),
    Unknown(Vec<String>),
}

pub fn get_command(args: &[String]) -> Command {
    let cmd = if args.is_empty() { "" } else { &args[0] };

    match cmd.to_lowercase().as_str() {
        "colour" | "c" => Command::Colour(args[1..].to_vec()),
        "region" | "r" => Command::Region(args[1..].to_vec()),
        "breathe" | "b" => Command::Breathe(args[1..].to_vec()),
        "cycle" | "cy" => Command::Cycle(args[1..].to_vec()),
        "help" | "h" | "?" => Command::Help(args[1..].to_vec()),
        _ => Command::Unknown(args.to_vec()),
    }
}

pub trait Run {
    fn run(&self, device: Device<GlobalContext>) -> Status;
}

impl Run for Command {
    fn run(&self, device: Device<GlobalContext>) -> Status {
        match self {
            Command::Colour(args) => colour_command(device, args),
            Command::Region(args) => region_command(device, args),
            Command::Breathe(args) => breathe_command(device, args),
            Command::Cycle(args) => cycle_command(device, args),
            Command::Help(args) => help_command(device, args),
            Command::Unknown(cmd) => {
                eprintln!("Uknown command: '{}'", cmd.join(" "));
                Status::Failure
            }
        }
    }
}

// ----------------------------------------------------------------------------

fn get_colour_or_red(args: &[String]) -> (u32, Status) {
    const RED: u32 = 0xff1010;

    match get_x11_colour(args) {
        Some(col) => (col, Status::Success),
        None => (RED, Status::Failure),
    }
}

fn colour_command(device: Device<GlobalContext>, args: &[String]) -> Status {
    let (colour, status) = get_colour_or_red(args);

    set_keyboard_colour(device, colour);

    status
}

fn region_command(device: Device<GlobalContext>, args: &[String]) -> Status {
    let mut status = Status::Failure;

    if !args.is_empty() {
        let region: KeyboardRegions = args[0].parse::<u8>().unwrap().into();

        let (colour, col_status) = get_colour_or_red(&args[1..]);

        set_region_colour(device, region as u8, colour);

        status = col_status;
    } else {
        eprintln!("At least one - 'region' ['colour'] - argument needed for 'region' command");
    }

    status
}

fn breathe_command(device: Device<GlobalContext>, args: &[String]) -> Status {
    let mut status = Status::Failure;

    if !args.is_empty() {
        let speed = limit_speed(args[0].parse::<u16>().unwrap());

        let (colour, col_status) = get_colour_or_red(&args[1..]);

        set_breathe(device, speed, colour);

        status = col_status;
    } else {
        eprintln!("At least one - 'speed' ['colour'] - argument needed for 'breathe' command");
    }

    status
}

fn cycle_command(device: Device<GlobalContext>, args: &[String]) -> Status {
    let mut status = Status::Failure;

    if args.len() == 1 {
        let speed = limit_speed(args[0].parse::<u16>().unwrap());

        set_cycle(device, speed);

        status = Status::Success;
    } else {
        eprintln!("One 'speed' argument needed for 'cycle' command");
    }

    status
}

fn help_command(_device: Device<GlobalContext>, _args: &[String]) -> Status {
    println!("g213-cols - version 0.2.0\n");

    // TODO: Give some device details
    println!("You do have a G213 keyboard ✅\n");

    println!("Please see -- https://crates.io/crates/g213_colours");

    Status::Failure
}
