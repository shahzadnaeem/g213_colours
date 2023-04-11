use dirs::home_dir;
use std::fs::File;
use std::io::{Read, Write};
use std::{env::args, process::ExitCode};

use g213_colours::commands::{get_command, Command, Run, Status};
use g213_colours::g213_keyboard::find_g213_keyboard;

const CONFIG_FILE: &str = ".g213-cols.json";

fn config_file_path() -> String {
    match home_dir() {
        Some(path) => format!("{}/{}", path.to_string_lossy(), CONFIG_FILE),
        None => String::new(),
    }
}

fn main() -> ExitCode {
    let device = find_g213_keyboard().expect("No G213 keyboard found, sorry!");

    let args = args().skip(1).collect::<Vec<_>>();

    let mut command = get_command(&args);

    // Use saved command if no command was specified and there is a saved one
    if let Command::Unknown(_args) = &command {
        let path = config_file_path();

        let f = File::open(path);

        if let Ok(mut fh) = f {
            let mut saved_cmd = String::new();

            fh.read_to_string(&mut saved_cmd)
                .expect("Unable to read saved command");

            command = serde_json::from_str(&saved_cmd).expect("Unable to use saved command");

            eprintln!("Using saved command: {:?}", command);
        }
    }

    let cmd_status = command.run(device);

    // Save the command if it was successful - for future use
    if let Status::Success = cmd_status {
        let ser_command = serde_json::to_string(&command).unwrap();
        let path = config_file_path();

        let mut f = File::create(path).expect("Unable to open config file for saving");

        Write::write_all(&mut f, ser_command.as_bytes()).expect("Unable to save command");
    }

    ExitCode::from(cmd_status as u8)
}
