use dirs::home_dir;
use libc::chown;
use std::ffi::CString;
use std::fmt::Display;
use std::fs::File;
use std::io::{Read, Write};

use rusb::{Device, GlobalContext};
use serde::{Deserialize, Serialize};
use users::{get_current_gid, get_current_uid};

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
    Info,
    Saved,
    Help(Vec<String>),
    Unknown(Vec<String>),
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Colour(args) => write!(f, "colour {}", args.join(" ")),
            Command::Region(args) => write!(f, "region {}", args.join(" ")),
            Command::Regions(args) => write!(f, "regions {}", args.join(" ")),
            Command::Breathe(args) => write!(f, "breathe {}", args.join(" ")),
            Command::Cycle(args) => write!(f, "cycle {}", args.join(" ")),
            Command::List(args) => write!(f, "list {}", args.join(" ")),
            Command::Info => write!(f, "info"),
            Command::Saved => write!(f, "saved"),
            Command::Help(args) => write!(f, "help {}", args.join(" ")),
            Command::Unknown(args) => write!(f, "unknown {}", args.join(" ")),
        }
    }
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
        "info" | "i" => Command::Info,
        "saved" | "s" => Command::Saved,
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
            Command::Info => info_command(device),
            Command::Saved => saved_command(),
            Command::Help(args) => help_command(args),
            Command::Unknown(args) => {
                eprintln!("Unknown command: {}", args.join(" "));
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
            Command::Help(args) => !args.is_empty(),
            Command::Unknown(args) => !args.is_empty(),
            _ => false,
        }
    }
}

// ----------------------------------------------------------------------------

const CONFIG_FILE: &str = ".g213-cols.json";

fn config_file_path() -> String {
    match home_dir() {
        Some(path) => format!("{}/{}", path.to_string_lossy(), CONFIG_FILE),
        None => String::new(),
    }
}

pub fn get_saved_command() -> Option<Command> {
    let path = config_file_path();

    let f = File::open(path);

    if let Ok(mut fh) = f {
        let mut saved_cmd = String::new();

        fh.read_to_string(&mut saved_cmd)
            .expect("Unable to read saved command");

        let command = serde_json::from_str(&saved_cmd).expect("Unable to use saved command");

        return Some(command);
    }

    None
}

pub fn set_file_ownership_to_me(path: String) {
    unsafe {
        let c_path = CString::new(path).unwrap();
        chown(c_path.as_ptr(), get_current_uid(), get_current_gid());
    }
}

pub fn save_command(command: &Command) {
    let ser_command = serde_json::to_string(&command).unwrap();
    let path = config_file_path();

    let mut f = File::create(&path).expect("Unable to open config file for saving");

    Write::write_all(&mut f, ser_command.as_bytes()).expect("Unable to save command");

    set_file_ownership_to_me(path);
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
    let mut status = Status::Failure;

    let names = x11_colour_names();
    let target: String = if args.is_empty() {
        "".to_string()
    } else {
        args[0].to_ascii_lowercase()
    };

    for name in &names {
        if target.is_empty() || name.contains(&target) {
            println!(
                "{} {:#08x}",
                name,
                get_x11_colour(&[name.to_string()]).unwrap()
            );
            status = Status::SuccessNoSave;
        }
    }

    status
}

fn saved_command() -> Status {
    let command = get_saved_command();

    match command {
        Some(cmd) => println!("Saved command: {}", cmd),
        None => println!("No currently saved command"),
    }

    Status::SuccessNoSave
}

fn info_command(device: &Device<GlobalContext>) -> Status {
    println!("Device bus:   {}", device.bus_number());
    println!("Device #:     {}", device.address());
    println!("Device speed: {:?}", device.speed());

    // Bit hacky, directly outputs info
    show_info(device);

    Status::SuccessNoSave
}

fn help_command(_args: &[String]) -> Status {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    println!(
        "\ng213-cols - version {} ― Found a G213 keyboard ✅",
        VERSION
    );

    println!(
        r#"
+ ----------------------------------------------------- + ------------------------------------------------------------- +
| Command                                               | Details                                                       |
| ----------------------------------------------------- + ------------------------------------------------------------- |
| Set whole keyboard colour                             |                                                               |
| `g213-cols colour`                                    | sets the default 'white'                                      |
| `g213-cols colour ffff00`                             | sets the specified hex colour, eg yellow                      |
|                                                       | NOTE: 3 digit hex colour conversion - 1af -> 11aaff           |
| `g213-cols colour lawn green`                         | sets the named X11 colour                                     |
| `g213-cols colour "alice blue"`                       | sets the named X11 colour - as a single argument              |
| `g213-cols colour dark_slate_blue`                    | sets the named X11 colour - underscores become spaces         |
| `g213-cols colour random`                             | sets a random colour                                          |
| Set the colour of a specific keyboard region - 1 to 5 |                                                               |
| `g213-cols region 2 [colour]`                         | sets the region to the [colour] as defined above              |
| Set the colours of all 5 keyboard regions             |                                                               |
| `g213-cols regions [colour colour ...]`               | sets the regions to the [colours] as defined above            |
|                                                       | last supplied colour is repeated to set all 5 regions         |
| Set 'breathe' mode                                    |                                                               |
| `g213-cols breathe 1000 [colour]`                     | sets the breathe time (in ms) for the [colour]                |
| Set 'cycle' mode                                      |                                                               |
| `g213-cols cycle 1000`                                | sets the cycle time (in ms) all colours                       |
| Use last successful saved command                     |                                                               |
| `g213-cols`                                           | Runs the last successful saved command from ~/.g213-cols.json |
| Show saved command                                    |                                                               |
| `g213-cols saved`                                     | Display the last successful saved command                     |
| Show keyboard info                                    |                                                               |
| `g213-cols info`                                      | Show USB info for keyboard                                    |
| List X11 colours                                      |                                                               |
| `g213-cols list [name]`                               | Show X11 colours that contain optional [name] or all colours  |
+ ----------------------------------------------------- + ------------------------------------------------------------- +

+ ------- + ------------ +
| Command | Abbreviation |
| ------- + ------------ |
| colour  | c            |
| region  | r            |
| regions | rs           |
| breathe | b            |
| cycle   | cy           |
| saved   | s            |
| info    | i            |
| list    | l            |
| help    | h or ?       |
+ ------- + ------------ +
"#
    );

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

    #[test]
    fn successful_status() {
        let res = Status::Success.successful() && Status::SuccessNoSave.successful();

        assert_eq!(res, true);
    }

    #[test]
    fn unsuccessful_status() {
        let res = Status::Failure.successful();

        assert_eq!(res, false);
    }
}
