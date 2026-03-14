use std::collections::HashMap;
use termcolor::Color;

pub struct ShellColor {
    color_map: HashMap<String, u8>,
}

impl ShellColor {
    pub fn new() -> Self {
        let mut color_map = HashMap::new();
        color_map.insert("reset".to_string(), 0);
        color_map.insert("black".to_string(), 30);
        color_map.insert("red".to_string(), 31);
        color_map.insert("green".to_string(), 32);
        color_map.insert("yellow".to_string(), 33);
        color_map.insert("blue".to_string(), 34);
        color_map.insert("magenta".to_string(), 35);
        color_map.insert("cyan".to_string(), 36);
        color_map.insert("white".to_string(), 37);
        Self { color_map }
    }

    pub fn is_valid_color(&self, color: &str) -> bool {
        if color.starts_with("\u{1b}[") || color.starts_with("\033[") {
            return true;
        }
        self.color_map.contains_key(color)
    }

    pub fn get_terminal_color(&self, color: &str) -> Option<Color> {
        match color {
            "black" => Some(Color::Black),
            "red" => Some(Color::Red),
            "green" => Some(Color::Green),
            "yellow" => Some(Color::Yellow),
            "blue" => Some(Color::Blue),
            "magenta" => Some(Color::Magenta),
            "cyan" => Some(Color::Cyan),
            "white" => Some(Color::White),
            _ => None,
        }
    }

    pub fn wrap_in_color(&self, to_wrap: &str, color: &str) -> String {
        if color.is_empty() {
            return to_wrap.to_string();
        }

        let ansi = if color.starts_with("\u{1b}[") || color.starts_with("\033[") {
            color.to_string()
        } else {
            if let Some(code) = self.color_map.get(color) {
                format!("\u{1b}[1;{}m", code)
            } else {
                return to_wrap.to_string();
            }
        };

        format!("{}{}\u{1b}[0m", ansi, to_wrap)
    }
}

impl Default for ShellColor {
    fn default() -> Self {
        Self::new()
    }
}
