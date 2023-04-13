use std::{env::args, process::ExitCode};

use g213_colours::commands::{
    get_command, get_saved_command, save_command, Command, Run, Status, Successful,
};
use g213_colours::g213_keyboard::find_g213_keyboard;

fn main() -> ExitCode {
    let device = find_g213_keyboard().expect("No G213 keyboard found, sorry!");

    let args = args().skip(1).collect::<Vec<_>>();

    let mut command = get_command(&args);

    // Use saved command if we have one and no command was specified
    if let Command::Unknown(_) = &command {
        if !command.has_args() {
            if let Some(cmd) = get_saved_command() {
                command = cmd;

                eprintln!("Using saved command: {:?}", command);
            }
        }
    }

    let cmd_status = command.run(&device);

    // Save the command for future use above, if it was successful
    if Status::Success == cmd_status {
        save_command(&command);
    }

    if cmd_status.successful() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(cmd_status as u8)
    }
}
