use std::collections::HashMap;

pub struct TextMapper {
    map: HashMap<char, [&'static str; 6]>,
}

impl TextMapper {
    pub fn new() -> Self {
        let mut mapper = TextMapper { map: HashMap::new() };

        mapper.map.insert('0', [
            r" _____ ",
            r"|  _  |",
            r"| |/' |",
            r"|  /| |",
            r"\ |_/ /",
            r" \___/ ",
        ]);

        mapper.map.insert('1', [
            r" __  ",
            r"/  | ",
            r"`| | ",
            r" | | ",
            r"_| |_",
            r"\___/",
        ]);

        mapper.map.insert('2', [
            r" _____ ",
            r"/ __  \",
            r"`' / /'",
            r"  / /  ",
            r"./ /___",
            r"\_____/",
        ]);

        mapper.map.insert('3', [
            r" _____ ",
            r"|____ |",
            r"    / /",
            r"    \ \",
            r".___/ /",
            r"\____/ ",
        ]);

        mapper.map.insert('4', [
            r"   ___ ",
            r"  /   |",
            r" / /| |",
            r"/ /_| |",
            r"\___  |",
            r"    |_/",
        ]);

        mapper.map.insert('5', [
            r" _____ ",
            r"|  ___|",
            r"|___ \ ",
            r"    \ \",
            r"/\__/ /",
            r"\____/ ",
        ]);

        mapper.map.insert('6', [
            r"  ____ ",
            r" / ___|",
            r"/ /___ ",
            r"| ___ \",
            r"| \_/ |",
            r"\_____/",
        ]);

        mapper.map.insert('7', [
            r" ______",
            r"|___  /",
            r"   / / ",
            r"  / /  ",
            r"./ /   ",
            r"\_/    ",
        ]);

        mapper.map.insert('8', [
            r" _____ ",
            r"|  _  |",
            r" \ V / ",
            r" / _ \ ",
            r"| |_| |",
            r"\_____/",
        ]);

        mapper.map.insert('9', [
            r" _____ ",
            r"|  _  |",
            r"| |_| |",
            r"\____ |",
            r".___/ /",
            r"\____/ ",
        ]);

        mapper.map.insert(':', [
            r"   ",
            r"(_)",
            r"   ",
            r" _ ",
            r"(_)",
            r"   ",
        ]);

        mapper.map.insert(' ', [
            r" ",
            r" ",
            r" ",
            r" ",
            r" ",
            r" ",
        ]);

        mapper
    }

    pub fn get_scan(&self, c: &char, line: usize) -> String {
        if let Some(a) = self.map.get(c) {
            String::from(a[line])
        } else {
            String::from("")
        }
    }

    pub fn get_string_scan(&self, s: &str, line: usize) -> String {
        s.chars().map(|c| {
            self.get_scan(&c, line)
        }).collect::<String>()
    }
}