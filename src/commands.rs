use crate::g213_keyboard::{set_breathe, set_cycle, set_whole_keyboard_colour};
use crate::x11_colours::get_x11_colour;
use rusb::{Device, GlobalContext};
use std::process::ExitCode;

pub fn colour_command(device: Device<GlobalContext>, args: &Vec<String>) -> ExitCode {
    const RED: u32 = 0xff1010;

    let (colour, exit_code) = match get_x11_colour(args) {
        Some(col) => (col, ExitCode::SUCCESS),
        None => (RED, ExitCode::FAILURE),
    };

    set_whole_keyboard_colour(device, colour);

    exit_code
}

pub fn breathe_command(device: Device<GlobalContext>, args: &Vec<String>) -> ExitCode {
    let mut exit_code = ExitCode::FAILURE;

    const RED: u32 = 0xff1010;

    if args.len() >= 2 {
        let speed = args[0].parse::<u16>().unwrap();
        let new_args = args.iter().skip(1).cloned().collect();

        let (colour, status) = match get_x11_colour(&new_args) {
            Some(col) => (col, ExitCode::SUCCESS),
            None => (RED, ExitCode::FAILURE),
        };

        set_breathe(device, speed, colour);

        exit_code = status
    } else {
        eprintln!("Not enough arguments for 'breathe' command")
    }

    exit_code
}

pub fn cycle_command(device: Device<GlobalContext>, args: &Vec<String>) -> ExitCode {
    let mut exit_code = ExitCode::FAILURE;

    if args.len() == 1 {
        let speed = args[0].parse::<u16>().unwrap();

        set_cycle(device, speed);

        exit_code = ExitCode::SUCCESS;
    } else {
        eprintln!("One argument needed for 'cycle' command");
    }

    exit_code
}

pub fn help_command(_device: Device<GlobalContext>, _args: &Vec<String>) -> ExitCode {
    println!("Help ...");

    // TODO: Give some device details
    println!("You do have a G213 keyboard ✅");

    // TODO: Usual help stuff

    ExitCode::SUCCESS
}
