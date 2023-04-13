use rusb::{Device, GlobalContext};
use serde::{Deserialize, Serialize};

use crate::g213_keyboard::{
    self, limit_speed, set_breathe, set_cycle, set_keyboard_colour, set_region_colour, show_info,
    KeyboardRegions,
};
use crate::x11_colours::{get_x11_colour, get_x11_colours, x11_colour_names};

#[repr(u8)]
#[derive(PartialEq)]
pub enum Status {
    Success = 0,
    Failure,
    SuccessNoSave,
}

pub trait Successful {
    fn successful(&self) -> bool;
}

impl Successful for Status {
    fn successful(&self) -> bool {
        Status::Success == *self || Status::SuccessNoSave == *self
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Colour(Vec<String>),
    Region(Vec<String>),
    Regions(Vec<String>),
    Breathe(Vec<String>),
    Cycle(Vec<String>),
    List(Vec<String>),
    Saved(Vec<String>),
    Help(Vec<String>),
    Unknown(Vec<String>),
}

pub fn get_command(args: &[String]) -> Command {
    let cmd = if args.is_empty() { "" } else { &args[0] };

    match cmd.to_lowercase().as_str() {
        "colour" | "c" => Command::Colour(args[1..].to_vec()),
        "region" | "r" => Command::Region(args[1..].to_vec()),
        "regions" | "rs" => Command::Regions(args[1..].to_vec()),
        "breathe" | "b" => Command::Breathe(args[1..].to_vec()),
        "cycle" | "cy" => Command::Cycle(args[1..].to_vec()),
        "list" | "l" => Command::List(args[1..].to_vec()),
        "saaved" | "s" => Command::Saved(args[1..].to_vec()),
        "help" | "h" | "?" => Command::Help(args[1..].to_vec()),
        _ => Command::Unknown(args.to_vec()),
    }
}

pub trait Run {
    fn run(&self, device: &Device<GlobalContext>) -> Status;
    fn has_args(&self) -> bool;
}

impl Run for Command {
    fn run(&self, device: &Device<GlobalContext>) -> Status {
        match self {
            Command::Colour(args) => colour_command(device, args),
            Command::Region(args) => region_command(device, args),
            Command::Regions(args) => regions_command(device, args),
            Command::Breathe(args) => breathe_command(device, args),
            Command::Cycle(args) => cycle_command(device, args),
            Command::List(args) => list_command(args),
            Command::Saved(args) => saved_command(args),
            Command::Help(args) => help_command(device, args),
            Command::Unknown(args) => {
                eprintln!("Uknown command: '{}'", args.join(" "));
                Status::SuccessNoSave
            }
        }
    }

    fn has_args(&self) -> bool {
        match self {
            Command::Colour(args) => !args.is_empty(),
            Command::Region(args) => !args.is_empty(),
            Command::Regions(args) => !args.is_empty(),
            Command::Breathe(args) => !args.is_empty(),
            Command::Cycle(args) => !args.is_empty(),
            Command::List(args) => !args.is_empty(),
            Command::Saved(args) => !args.is_empty(),
            Command::Help(args) => !args.is_empty(),
            Command::Unknown(_) => false,
        }
    }
}

// ----------------------------------------------------------------------------

const RED: u32 = 0xff1010;

fn get_colour_or_red(args: &[String]) -> (u32, Status) {
    match get_x11_colour(args) {
        Some(col) => (col, Status::Success),
        None => (RED, Status::Failure),
    }
}

fn get_colours_or_red(args: &[String], num: u8) -> (Vec<u32>, Status) {
    match get_x11_colours(args, num) {
        Some(cols) => (cols, Status::Success),
        None => (vec![RED; num as usize], Status::Failure),
    }
}

fn colour_command(device: &Device<GlobalContext>, args: &[String]) -> Status {
    let (colour, status) = get_colour_or_red(args);

    set_keyboard_colour(device, colour);

    status
}

fn region_command(device: &Device<GlobalContext>, args: &[String]) -> Status {
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

fn regions_command(device: &Device<GlobalContext>, args: &[String]) -> Status {
    let (colours, status) = get_colours_or_red(args, g213_keyboard::NUM_REGIONS);

    colours
        .iter()
        .enumerate()
        .for_each(|(region, colour)| set_region_colour(device, (region + 1) as u8, *colour));

    status
}

fn breathe_command(device: &Device<GlobalContext>, args: &[String]) -> Status {
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

fn cycle_command(device: &Device<GlobalContext>, args: &[String]) -> Status {
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

fn list_command(args: &[String]) -> Status {
    let names = x11_colour_names();

    for name in names {
        if args.is_empty() || name.contains(&args[0]) {
            println!(
                "{} {:#08x}",
                name,
                get_x11_colour(&[name.to_string()]).unwrap()
            );
        }
    }

    Status::SuccessNoSave
}

fn saved_command(args: &[String]) -> Status {
    println!("TODO: Show saved command.");

    Status::SuccessNoSave
}

fn help_command(device: &Device<GlobalContext>, _args: &[String]) -> Status {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    println!("g213-cols - version {}\n", VERSION);
    println!("You do have a G213 keyboard ✅\n");

    println!("Device bus:   {}", device.bus_number());
    println!("Device #:     {}", device.address());
    println!("Device speed: {:?}", device.speed());

    // Bit hacky, directly outputs info
    show_info(device);

    println!("\nPlease see -- https://crates.io/crates/g213_colours");

    Status::SuccessNoSave
}

#[cfg(test)]
mod commands_tests {

    use super::*;

    fn to_string_vec(words: Vec<&str>) -> Vec<String> {
        words.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn colour_command() {
        let args = to_string_vec(vec!["colour"]);

        let cmd = get_command(&args);

        assert!(match cmd {
            Command::Colour(_) => true,
            _ => false,
        });

        assert!(!cmd.has_args());
    }

    #[test]
    fn colour_command_with_args() {
        let args = to_string_vec(vec!["colour", "0xff00ff"]);

        let cmd = get_command(&args);

        assert!(match cmd {
            Command::Colour(_) => true,
            _ => false,
        });

        assert!(cmd.has_args());
    }
}
