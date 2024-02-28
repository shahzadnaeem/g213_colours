use std::collections::HashMap;

use rand::random;

const X11_COLOURS: &str = include_str!("../rgb.txt");

type ColourLookup = HashMap<String, u32>;
type ColourNames = Vec<String>;

lazy_static! {
    static ref DEFINITIONS: Vec<(String, u32)> = parse_x11_colours();
    static ref COLOUR_LOOKUP: ColourLookup = {
        let mut map = HashMap::new();

        for def in DEFINITIONS.iter() {
            map.insert(def.0.clone(), def.1);
        }

        map
    };
    static ref COLOUR_NAMES: ColourNames = DEFINITIONS.iter().map(|d| d.0.clone()).collect();
}

fn parse_x11_colours() -> Vec<(String, u32)> {
    let lines = X11_COLOURS
        .lines()
        .filter(|l| !l.starts_with('#') && !l.is_empty());

    let definitions: Vec<_> = lines
        .map(|l| {
            let parts: Vec<_> = l.split_ascii_whitespace().collect();

            let r = parts[0].parse::<u32>().unwrap();
            let g = parts[1].parse::<u32>().unwrap();
            let b = parts[2].parse::<u32>().unwrap();

            let name = if parts.len() == 4 {
                parts[3].to_ascii_lowercase()
            } else if parts.len() == 5 {
                [parts[3], parts[4]].join(" ").to_ascii_lowercase()
            } else {
                [parts[3], parts[4], parts[5]]
                    .join(" ")
                    .to_ascii_lowercase()
            };

            (name, r * 256 * 256 + g * 256 + b)
        })
        .collect();

    definitions
}

fn get_colour_def(name: &str) -> Option<u32> {
    let name_lc = name.to_ascii_lowercase().replace('_', " ");
    COLOUR_LOOKUP.get(&name_lc).copied()
}

fn random_colour() -> u32 {
    random::<u32>() & 0x00ffffff
}

fn random_x11_colour() -> (&'static String, u32) {
    let n = (random::<f32>() * (NUM_X11_COLOURS as f32)) as u32;

    let name = &COLOUR_NAMES[n as usize];

    (name, *COLOUR_LOOKUP.get(name).unwrap())
}

fn adjust_3_digit_colour(colour: u32) -> u32 {
    if colour & 0xfff000 == 0 {
        let d1: u32 = (colour & 0xf00) >> 8;
        let d2: u32 = (colour & 0xf0) >> 4;
        let d3: u32 = colour & 0xf;

        (d1 * 16 + d1) << 16 | (d2 * 16 + d2) << 8 | (d3 * 16 + d3)
    } else {
        colour
    }
}

pub const NUM_X11_COLOURS: usize = 759;
pub const DEFAULT_WHITE: u32 = 0xffd0c0;
const RANDOM: &str = "random";
const RANDOM_X11: &str = "randomx11";

pub fn x11_colour_names() -> Vec<&'static String> {
    COLOUR_NAMES.iter().collect()
}

pub fn get_x11_colour(args: &[String]) -> Option<u32> {
    let mut colour: Option<u32> = None;

    if args.is_empty() {
        colour = Some(DEFAULT_WHITE);
    } else if args.len() == 1 {
        if let Ok(mut numeric_col) = u32::from_str_radix(args[0].trim_start_matches("0x"), 16) {
            numeric_col &= 0xffffff;

            let digits = args[0].trim_start_matches("0x").len();
            if digits == 3 {
                numeric_col = adjust_3_digit_colour(numeric_col);
            }
            colour = Some(numeric_col);
        } else if args[0].to_ascii_lowercase() == RANDOM {
            colour = Some(random_colour())
        } else if args[0].to_ascii_lowercase() == RANDOM_X11 {
            colour = Some(random_x11_colour().1)
        } else if let Some(named_col) = get_colour_def(&args[0]) {
            colour = Some(named_col);
        }
    } else {
        let name = args.join(" ");

        if let Some(named_col) = get_colour_def(&name) {
            colour = Some(named_col)
        }
    }

    colour
}

pub fn get_x11_colours(args: &[String], num: u8) -> Option<Vec<u32>> {
    let mut col_str: String = "".to_string();
    let mut n: u8 = 0;
    let mut cols = Vec::<u32>::new();
    let mut last_col_str: String = "".to_string();

    if !args.is_empty() {
        for arg in args {
            col_str += arg;

            if let Some(col) = get_x11_colour(&[col_str.clone()]) {
                cols.push(col);
                n += 1;

                last_col_str = col_str.clone();
                col_str.truncate(0);
            }

            if n == num {
                break;
            };
        }
    } else {
        cols = vec![DEFAULT_WHITE; num as usize];
        n = num;
    }

    if !cols.is_empty() && n < num {
        while n != num {
            cols.push(get_x11_colour(&[last_col_str.clone()]).unwrap());
            n += 1;
        }
    }

    if n == num {
        Some(cols)
    } else {
        None
    }
}

#[cfg(test)]
mod x11_colours_tests {
    use crate::g213_keyboard::NUM_REGIONS;

    use super::*;

    #[test]
    fn num_colours() {
        assert_eq!(COLOUR_LOOKUP.len(), NUM_X11_COLOURS);
    }

    #[test]
    fn get_def_white() {
        assert_eq!(get_colour_def("white"), Some(0xffffff));
    }

    #[test]
    fn get_def_alice_blue() {
        assert_eq!(get_colour_def("alice blue"), Some(0xf0f8ff));
    }

    #[test]
    fn get_def_alice_blue_mixed_case() {
        assert_eq!(get_colour_def("ALICE blue"), Some(0xf0f8ff));
    }

    #[test]
    fn get_def_aliceblue() {
        assert_eq!(get_colour_def("AliceBlue"), Some(0xf0f8ff));
    }

    #[test]
    fn get_def_aliceblue_mixed_case() {
        assert_eq!(get_colour_def("AlicEBLUE"), Some(0xf0f8ff));
    }

    #[test]
    fn def_none_for_bluuuu() {
        assert_eq!(get_colour_def("bluuuuu"), None);
    }

    #[test]
    fn def_none_for_blue_uuu() {
        assert_eq!(get_colour_def("blue uuu"), None);
    }

    #[test]
    fn get_def_first_snow() {
        assert_eq!(get_colour_def("snow"), Some(0xfffafa));
    }

    #[test]
    fn get_def_last_light_green() {
        assert_eq!(get_colour_def("LightGreen"), Some(0x90ee90));
    }

    #[test]
    fn get_def_medium_violet_red() {
        assert_eq!(get_colour_def("mediumvioletRED"), Some(0xc71585));
    }

    #[test]
    fn get_colour_default() {
        let args = Vec::new();

        assert_eq!(get_x11_colour(&args), Some(DEFAULT_WHITE));
    }

    fn to_string_vec(words: Vec<&str>) -> Vec<String> {
        words.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn get_colour_medium_violet_red() {
        let args = to_string_vec(vec!["Medium", "Violet", "Red"]);

        assert_eq!(get_x11_colour(&args), Some(0xc71585));
    }

    #[test]
    fn get_colour_alt_medium_violet_red() {
        let args = to_string_vec(vec!["Medium", "Violet Red"]);

        assert_eq!(get_x11_colour(&args), Some(0xc71585));
    }

    #[test]
    fn get_colour_with_underscores() {
        let args = to_string_vec(vec!["light_goldenrod", "yellow"]);

        assert_eq!(get_x11_colour(&args), Some(0xfafad2));
    }

    #[test]
    fn get_colour_none_for_x11_uknown() {
        let args = to_string_vec(vec!["not", "a_colour"]);

        assert_eq!(get_x11_colour(&args), None);
    }

    #[test]
    fn get_colour_none_for_x11_too_many_args() {
        let args = to_string_vec(vec!["no", "four", "word", "colours"]);

        assert_eq!(get_x11_colour(&args), None);
    }

    #[test]
    fn get_colour_hex() {
        let args = to_string_vec(vec!["ff0055"]);

        assert_eq!(get_x11_colour(&args), Some(0xff0055));
    }

    #[test]
    fn get_colour_hex_4digits() {
        let args = to_string_vec(vec!["ff00"]);

        assert_eq!(get_x11_colour(&args), Some(0xff00));
    }

    #[test]
    fn get_colour_hex_3digits_fs() {
        let args = to_string_vec(vec!["fff"]);

        assert_eq!(get_x11_colour(&args), Some(0xffffff));
    }

    #[test]
    fn get_colour_hex_3digits_1s() {
        let args = to_string_vec(vec!["111"]);

        assert_eq!(get_x11_colour(&args), Some(0x111111));
    }

    #[test]
    fn get_colour_hex_2digits() {
        let args = to_string_vec(vec!["f1"]);

        assert_eq!(get_x11_colour(&args), Some(0xf1));
    }

    #[test]
    fn get_colour_0x_hex() {
        let args = to_string_vec(vec!["0xbeefee"]);

        assert_eq!(get_x11_colour(&args), Some(0xbeefee));
    }

    #[test]
    fn get_colour_0x_hex_max_3_bytes() {
        let args = to_string_vec(vec!["0x1fbeefee"]);

        assert_eq!(get_x11_colour(&args), Some(0xbeefee));
    }

    #[test]
    fn get_random_colour() {
        let col = random_colour();

        assert!(col <= 0xffffff);
    }

    #[test]
    fn get_random_x11_colour() {
        let col = random_x11_colour();

        assert!(!col.0.is_empty());
        assert!(col.1 <= 0xffffff);
    }

    #[test]
    fn get_5_colours() {
        let args = to_string_vec(vec!["red", "blue", "green", "white", "black"]);

        assert_eq!(
            get_x11_colours(&args, 5),
            Some(vec![0xff0000, 0xff, 0xff00, 0xffffff, 0x0])
        );
    }

    #[test]
    fn get_5_colours_empty_args() {
        let args = to_string_vec(vec![]);

        assert_eq!(get_x11_colours(&args, 5), Some(vec![DEFAULT_WHITE; 5]));
    }

    #[test]
    fn get_5_colours_check_padding() {
        let args = to_string_vec(vec!["red", "blue", "green", "white"]);

        assert_eq!(
            get_x11_colours(&args, 5),
            Some(vec![0xff0000, 0xff, 0xff00, 0xffffff, 0xffffff])
        );
    }

    #[test]
    fn get_5_colours_single_random() {
        let args = to_string_vec(vec!["random"]);

        let colours = get_x11_colours(&args, NUM_REGIONS).unwrap();

        assert_eq!(colours.len(), NUM_REGIONS as usize);

        let mut differences: u32 = 0;

        for i in 0..NUM_REGIONS {
            for j in 0..NUM_REGIONS {
                if i != j && colours[i as usize] != colours[j as usize] {
                    differences += 1;
                }
            }
        }

        // Should be a higher threshold I guess
        assert!(differences > NUM_REGIONS as u32);
    }

    #[test]
    fn get_2_colours_multi_word() {
        let args = to_string_vec(vec!["alice", "blue", "medium", "violet", "red"]);

        assert_eq!(get_x11_colours(&args, 2), Some(vec![0xf0f8ff, 0xc71585]));
    }

    #[test]
    fn adjust_3_digit_111() {
        assert_eq!(adjust_3_digit_colour(0x111), 0x111111);
    }

    #[test]
    fn adjust_3_digit_321() {
        assert_eq!(adjust_3_digit_colour(0x321), 0x332211);
    }

    #[test]
    fn adjust_3_digit_345678_unchanged() {
        assert_eq!(adjust_3_digit_colour(0x345678), 0x345678);
    }

    #[test]
    fn adjust_3_digit_35678_unchanged() {
        assert_eq!(adjust_3_digit_colour(0x35678), 0x35678);
    }

    #[test]
    fn adjust_3_digit_3678_unchanged() {
        assert_eq!(adjust_3_digit_colour(0x3678), 0x3678);
    }
}
