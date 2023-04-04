use std::collections::HashMap;

const X11_COLOURS: &str = include_str!("../rgb.txt");

type ColourLookup = HashMap<String, u32>;

lazy_static! {
    static ref COLOUR_LOOKUP: ColourLookup = {
        let definitions = rgb_to_rust();

        let mut map = HashMap::new();

        for def in definitions.iter() {
            map.insert(def.0.clone(), def.1.clone());
        }

        map
    };
}

fn rgb_to_rust() -> Vec<(String, u32)> {
    let lines = X11_COLOURS
        .lines()
        .filter(|l| !l.contains("#") && l.len() != 0);

    let definitions: Vec<_> = lines
        .map(|l| {
            let parts: Vec<_> = l.split_ascii_whitespace().collect();

            let r = u32::from_str_radix(parts[0], 10).unwrap();
            let g = u32::from_str_radix(parts[1], 10).unwrap();
            let b = u32::from_str_radix(parts[2], 10).unwrap();

            let name = if parts.len() == 4 {
                parts[3].to_ascii_lowercase()
            } else {
                [parts[3], parts[4]].join(" ").to_ascii_lowercase()
            };

            (name, r * 256 * 256 + g * 256 + b)
        })
        .collect();

    definitions
}

pub fn get_colour_def(name: &str) -> Option<u32> {
    let name_lc = name.to_ascii_lowercase();
    COLOUR_LOOKUP.get(&name_lc).copied()
}

#[cfg(test)]
mod x11_colours_tests {
    use super::*;

    #[test]
    fn get_white() {
        assert_eq!(get_colour_def("white"), Some(0xffffff));
    }

    #[test]
    fn get_alice_blue() {
        assert_eq!(get_colour_def("alice blue"), Some(0xf0f8ff));
    }

    #[test]
    fn get_alice_blue_uc() {
        assert_eq!(get_colour_def("ALICE Blue"), Some(0xf0f8ff));
    }

    #[test]
    fn get_aliceblue() {
        assert_eq!(get_colour_def("AliceBlue"), Some(0xf0f8ff));
    }

    #[test]
    fn none_for_bluuuu() {
        assert_eq!(get_colour_def("bluuuuu"), None);
    }

    #[test]
    fn none_for_blue_uuu() {
        assert_eq!(get_colour_def("blue_uuu"), None);
    }
}
