//! # Font
//!
//! This module contains a struct called [TextMapper] which implements helper functions for converting
//! given strings into lines of ASCII art fonts

use std::collections::HashMap;

/// Contains a map for converting various characters into lines of ASCII art
pub struct TextMapper {
    map: HashMap<char, [&'static str; 6]>,
}

impl TextMapper {
    /// Create a new [TextMapper] and initialize the font mappings
    pub fn new() -> Self {
        let mut mapper = TextMapper {
            map: HashMap::new(),
        };

        mapper.map.insert(
            '0',
            [
                r" _____ ", r"|  _  |", r"| |/' |", r"|  /| |", r"\ |_/ /", r" \___/ ",
            ],
        );

        mapper.map.insert(
            '1',
            [r" __  ", r"/  | ", r"`| | ", r" | | ", r"_| |_", r"\___/"],
        );

        mapper.map.insert(
            '2',
            [
                r" _____ ", r"/ __  \", r"`' / /'", r"  / /  ", r"./ /___", r"\_____/",
            ],
        );

        mapper.map.insert(
            '3',
            [
                r" _____ ", r"|____ |", r"    / /", r"    \ \", r".___/ /", r"\____/ ",
            ],
        );

        mapper.map.insert(
            '4',
            [
                r"   ___ ", r"  /   |", r" / /| |", r"/ /_| |", r"\___  |", r"    |_/",
            ],
        );

        mapper.map.insert(
            '5',
            [
                r" _____ ", r"|  ___|", r"|___ \ ", r"    \ \", r"/\__/ /", r"\____/ ",
            ],
        );

        mapper.map.insert(
            '6',
            [
                r"  ____ ", r" / ___|", r"/ /___ ", r"| ___ \", r"| \_/ |", r"\_____/",
            ],
        );

        mapper.map.insert(
            '7',
            [
                r" ______", r"|___  /", r"   / / ", r"  / /  ", r"./ /   ", r"\_/    ",
            ],
        );

        mapper.map.insert(
            '8',
            [
                r" _____ ", r"|  _  |", r" \ V / ", r" / _ \ ", r"| |_| |", r"\_____/",
            ],
        );

        mapper.map.insert(
            '9',
            [
                r" _____ ", r"|  _  |", r"| |_| |", r"\____ |", r".___/ /", r"\____/ ",
            ],
        );

        mapper
            .map
            .insert(':', [r"   ", r"(_)", r"   ", r" _ ", r"(_)", r"   "]);

        mapper.map.insert(' ', [r" ", r" ", r" ", r" ", r" ", r" "]);

        mapper
    }

    /// Get a single line of the ASCII art for a given character
    ///
    /// # Arguments
    ///
    /// * `c` - The character being mapped
    /// * `line` - Which line of the ASCII art to return
    ///
    /// # Returns
    ///
    /// A line of ASCII art
    pub fn get_scan(&self, c: &char, line: usize) -> String {
        if let Some(a) = self.map.get(c) {
            String::from(a[line])
        } else {
            String::from("")
        }
    }

    /// Get a single line of ASCII art for a given string
    ///
    /// # Arguments
    ///
    /// * `s` - The string being mapped
    /// * `line` - Which line of the ASCII art to return
    ///
    /// # Returns
    ///
    /// A line of ASCII art for all characters in the string
    pub fn get_string_scan(&self, s: &str, line: usize) -> String {
        // Get the line for each character and concatenate into a single string
        s.chars()
            .map(|c| self.get_scan(&c, line))
            .collect::<String>()
    }
}
