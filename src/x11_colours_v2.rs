//
// Version 2 of x11_colours - aiming for minimal copying
//

use std::collections::HashMap;

const X11_COLOURS: &str = include_str!("../rgb.txt");

type ColourLookup = HashMap<&'static str, u32>;
type ColourNames = Vec<&'static str>;

lazy_static! {
    static ref DEFINITIONS: Vec<(&'static str, u32)> = parse_x11_colours();
    static ref COLOUR_LOOKUP: ColourLookup = {
        let mut map = HashMap::new();

        for def in DEFINITIONS.iter() {
            map.insert(def.0, def.1);
        }

        map
    };
    static ref COLOUR_NAMES: ColourNames = DEFINITIONS.iter().map(|d| d.0).collect();
}

fn next_non_whitespace(l: &str, start: usize) -> Option<usize> {
    if let Some(pos) = &l[start..]
        .as_bytes()
        .iter()
        .position(|ch| !ch.is_ascii_whitespace())
    {
        return Some(start + *pos);
    }

    None
}

fn parse_x11_colours() -> Vec<(&'static str, u32)> {
    X11_COLOURS
        .lines()
        .filter(|l| !l.starts_with('#') && !l.is_empty())
        .map(|l| {
            let defn = &l[0..11];
            let name = &l[next_non_whitespace(l, 12).unwrap()..];

            let components = defn.split_ascii_whitespace().collect::<Vec<_>>();

            let r = components[0].parse::<u32>().unwrap();
            let g = components[1].parse::<u32>().unwrap();
            let b = components[2].parse::<u32>().unwrap();

            (name, r * 256 * 256 + g * 256 + b)
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod x11_2_tests {
    use crate::x11_colours::NUM_X11_COLOURS;

    use super::*;

    #[test]
    fn non_ws_first() {
        assert_eq!(next_non_whitespace("Hello", 0), Some(0));
    }

    #[test]
    fn non_ws_next() {
        assert_eq!(next_non_whitespace("Hello  there", 5), Some(7));
    }

    #[test]
    fn non_ws_none() {
        assert_eq!(next_non_whitespace("Hello  ", 5), None);
    }

    #[test]
    fn non_ws_end_none() {
        assert_eq!(next_non_whitespace("Hello", 5), None);
    }

    #[test]
    fn num_colours() {
        assert_eq!(COLOUR_LOOKUP.len(), NUM_X11_COLOURS);
    }

    #[test]
    fn get_snow() {
        assert_eq!(COLOUR_LOOKUP.get("snow"), Some(&0xfffafa));
    }
}
