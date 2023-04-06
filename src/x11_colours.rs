use std::collections::HashMap;

const X11_COLOURS: &str = include_str!("../rgb.txt");

type ColourLookup = HashMap<String, u32>;

lazy_static! {
    static ref COLOUR_LOOKUP: ColourLookup = {
        let definitions = parse_x11_colours();

        let mut map = HashMap::new();

        for def in definitions.iter() {
            map.insert(def.0.clone(), def.1);
        }

        map
    };
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

const WHITE: u32 = 0xffd0c0;

// TODO: Simplify by simply joining all args if more than 1?

pub fn get_x11_colour(args: &[String]) -> Option<u32> {
    let mut colour: Option<u32> = None;

    if args.is_empty() {
        colour = Some(WHITE);
    } else if args.len() == 1 {
        if let Ok(numeric_col) = u32::from_str_radix(args[0].trim_start_matches("0x"), 16) {
            colour = Some(numeric_col);
        } else if let Some(named_col) = get_colour_def(&args[0]) {
            colour = Some(named_col);
        }
    } else if args.len() == 2 {
        let name = format!("{} {}", &args[0], &args[1]);

        if let Some(named_col) = get_colour_def(&name) {
            colour = Some(named_col)
        }
    } else if args.len() == 3 {
        let name = format!("{} {} {}", &args[0], &args[1], &args[2]);

        if let Some(named_col) = get_colour_def(&name) {
            colour = Some(named_col)
        }
    }

    colour
}

#[cfg(test)]
mod x11_colours_tests {
    use super::*;

    #[test]
    fn num_colours() {
        const NUM_COLOURS: usize = 752;

        assert_eq!(COLOUR_LOOKUP.len(), NUM_COLOURS);
    }

    #[test]
    fn get_white() {
        assert_eq!(get_colour_def("white"), Some(0xffffff));
    }

    #[test]
    fn get_alice_blue() {
        assert_eq!(get_colour_def("alice blue"), Some(0xf0f8ff));
    }

    #[test]
    fn get_alice_blue_mixed_case() {
        assert_eq!(get_colour_def("ALICE blue"), Some(0xf0f8ff));
    }

    #[test]
    fn get_aliceblue() {
        assert_eq!(get_colour_def("AliceBlue"), Some(0xf0f8ff));
    }

    #[test]
    fn get_aliceblue_mixed_case() {
        assert_eq!(get_colour_def("AlicEBLUE"), Some(0xf0f8ff));
    }

    #[test]
    fn none_for_bluuuu() {
        assert_eq!(get_colour_def("bluuuuu"), None);
    }

    #[test]
    fn none_for_blue_uuu() {
        assert_eq!(get_colour_def("blue uuu"), None);
    }

    #[test]
    fn get_first_snow() {
        assert_eq!(get_colour_def("snow"), Some(0xfffafa));
    }

    #[test]
    fn get_last_light_green() {
        assert_eq!(get_colour_def("LightGreen"), Some(0x90ee90));
    }

    #[test]
    fn get_medium_violet_red() {
        assert_eq!(get_colour_def("mediumvioletRED"), Some(0xc71585));
    }

    #[test]
    fn get_x11_default() {
        let args = Vec::new();

        assert_eq!(get_x11_colour(&args), Some(WHITE));
    }

    #[test]
    fn get_x11_medium_violet_red() {
        let args = vec!["Medium", "Violet", "Red"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        assert_eq!(get_x11_colour(&args), Some(0xc71585));
    }

    #[test]
    fn get_x11_alt_medium_violet_red() {
        let args = vec!["Medium", "Violet Red"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        assert_eq!(get_x11_colour(&args), Some(0xc71585));
    }

    #[test]
    fn get_x11_with_underscores() {
        let args = vec!["light_goldenrod", "yellow"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        assert_eq!(get_x11_colour(&args), Some(0xfafad2));
    }

    #[test]
    fn none_for_x11_uknown() {
        let args = vec!["not", "a_colour"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        assert_eq!(get_x11_colour(&args), None);
    }

    #[test]
    fn none_for_x11_too_many_args() {
        let args = vec!["four", "is", "too", "many"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        assert_eq!(get_x11_colour(&args), None);
    }

    #[test]
    fn none_for_x11_too_many_more_args() {
        let args = vec!["six", "is", "also", "too", "many", "args"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        assert_eq!(get_x11_colour(&args), None);
    }

    #[test]
    fn get_x11_hex() {
        let args = vec!["ff0055"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        assert_eq!(get_x11_colour(&args), Some(0xff0055));
    }

    #[test]
    fn get_x11_hex_4digits() {
        let args = vec!["ff00"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        assert_eq!(get_x11_colour(&args), Some(0xff00));
    }

    #[test]
    fn get_x11_hex_2digits() {
        let args = vec!["f1"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        assert_eq!(get_x11_colour(&args), Some(0xf1));
    }

    #[test]
    fn get_x11_0x_hex() {
        let args = vec!["0xbeefee"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        assert_eq!(get_x11_colour(&args), Some(0xbeefee));
    }
}
