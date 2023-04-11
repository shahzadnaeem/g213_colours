use dirs::home_dir;
use std::fs::File;
use std::io::Write;
use std::{env::args, process::ExitCode};

use g213_colours::commands::{get_command, Run, Status};
use g213_colours::g213_keyboard::find_g213_keyboard;

const CONFIG_FILE: &str = ".g213-cols.json";
const COMMAND: &str = "command";

fn config_file_path() -> String {
    match home_dir() {
        Some(path) => format!("{}/{}", path.to_string_lossy(), CONFIG_FILE),
        None => String::new(),
    }
}

fn main() -> ExitCode {
    let device = find_g213_keyboard().expect("No G213 keyboard found, sorry!");

    let args = args().skip(1).collect::<Vec<_>>();
    let command = get_command(&args);

    let cmd_status = command.run(device);

    match cmd_status {
        Status::Success => {
            let ser_command = serde_json::to_string(&command).unwrap();
            let path = config_file_path();

            println!("Command: {} - path: {}", ser_command, path);

            let mut f = File::create(path).expect("Unable to open config file for saving");

            let written = f
                .write(ser_command.as_bytes())
                .expect("Unable to save command");

            println!("Command saved: {} bytes written", written);
        }
        _ => {}
    }

    ExitCode::from(cmd_status as u8)
}
