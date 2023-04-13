use dirs::home_dir;
use libc::chown;
use std::ffi::CString;
use std::fs::File;
use std::io::{Read, Write};
use std::{env::args, process::ExitCode};
use users::{get_current_gid, get_current_uid};

use g213_colours::commands::{get_command, Command, Run, Status, Successful};
use g213_colours::g213_keyboard::find_g213_keyboard;

const CONFIG_FILE: &str = ".g213-cols.json";

fn config_file_path() -> String {
    match home_dir() {
        Some(path) => format!("{}/{}", path.to_string_lossy(), CONFIG_FILE),
        None => String::new(),
    }
}

fn get_saved_command() -> Option<Command> {
    let path = config_file_path();

    let f = File::open(path);

    if let Ok(mut fh) = f {
        let mut saved_cmd = String::new();

        fh.read_to_string(&mut saved_cmd)
            .expect("Unable to read saved command");

        let command = serde_json::from_str(&saved_cmd).expect("Unable to use saved command");

        eprintln!("Using saved command: {:?}", command);

        return Some(command);
    }

    None
}

fn set_file_ownership_to_me(path: String) {
    unsafe {
        let c_path = CString::new(path).unwrap();
        chown(c_path.as_ptr(), get_current_uid(), get_current_gid());
    }
}

fn save_command(command: &Command) {
    let ser_command = serde_json::to_string(&command).unwrap();
    let path = config_file_path();

    let mut f = File::create(&path).expect("Unable to open config file for saving");

    Write::write_all(&mut f, ser_command.as_bytes()).expect("Unable to save command");

    set_file_ownership_to_me(path);
}

fn main() -> ExitCode {
    let device = find_g213_keyboard().expect("No G213 keyboard found, sorry!");

    let args = args().skip(1).collect::<Vec<_>>();

    let mut command = get_command(&args);

    // Use saved command if we have one and no command was specified
    if let Command::Unknown(args) = &command {
        if args.is_empty() {
            if let Some(cmd) = get_saved_command() {
                command = cmd;
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
