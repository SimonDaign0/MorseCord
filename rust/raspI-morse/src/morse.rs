pub mod mrs_code {
    pub fn decode(input: &str) -> Option<char> {
        match input {
            // Letters
            ".-" => Some('a'),
            "-..." => Some('b'),
            "-.-." => Some('c'),
            "-.." => Some('d'),
            "." => Some('e'),
            "..-." => Some('f'),
            "--." => Some('g'),
            "...." => Some('h'),
            ".." => Some('i'),
            ".---" => Some('j'),
            "-.-" => Some('k'),
            ".-.." => Some('l'),
            "--" => Some('m'),
            "-." => Some('n'),
            "---" => Some('o'),
            ".--." => Some('p'),
            "--.-" => Some('q'),
            ".-." => Some('r'),
            "..." => Some('s'),
            "-" => Some('t'),
            "..-" => Some('u'),
            "...-" => Some('v'),
            ".--" => Some('w'),
            "-..-" => Some('x'),
            "-.--" => Some('y'),
            "--.." => Some('z'),

            // Special
            "....-" => Some('<'), // BACKSPACE
            "....." => Some('>'), // SPACE

            _ => None,
        }
    }
}
