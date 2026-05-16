pub enum ColorPlace {
    Foreground = 38,
    Background = 48,
}

pub struct RGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Style<'a> {
    pub name: &'a str,
}

impl<'a> Style<'a> {
    fn to_id(&self) -> u8 {
        match self.name {
            "reset" => 0,
            "bold" => 1,
            "dim" => 2,
            "italic" => 3,
            "underline" => 4,
            "blink" => 5,
            "invert" => 7,
            _ => 0,
        }
    }
}

pub enum ColorType<'a> {
    RGB(RGBColor),
    Id(&'a str),
    Style(Style<'a>),
}

pub fn generate(place: Option<ColorPlace>, r#type: ColorType) -> Option<String> {
    match r#type {
        ColorType::Style(style) => Some(format!("\x1b[{}m", style.to_id())),
        ColorType::Id(name) => {
            let color_id = COLORS
                .iter()
                .find(|&&color| color.0 == name)
                .map(|&color| color.1)?;
            Some(format!(
                "\x1b[{};5;{}m",
                place.unwrap_or(ColorPlace::Foreground) as u8,
                color_id
            ))
        }
        ColorType::RGB(color) => Some(format!(
            "\x1b[{};2;{};{};{}m",
            place.unwrap_or(ColorPlace::Foreground) as u8,
            color.r,
            color.g,
            color.b
        )),
    }
}

// Format: (color name, id)
pub static COLORS: &[(&str, u8)] = &[
    ("black", 0),
    ("red", 1),
    ("green", 2),
    ("yellow", 3),
    ("blue", 4),
    ("magenta", 5),
    ("cyan", 6),
    ("white", 7),
    ("bright-black", 8),
    ("bright-red", 9),
    ("bright-green", 10),
    ("bright-yellow", 11),
    ("bright-blue", 12),
    ("bright-magenta", 13),
    ("bright-cyan", 14),
    ("bright-white", 15),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_find() {
        let color = generate(None, ColorType::Style(Style { name: "bold" })).unwrap();
        assert_eq!(color, "\x1b[1m");
    }

    #[test]
    fn test_generate() {
        let color_foreground =
            generate(Some(ColorPlace::Foreground), ColorType::Id("cyan")).unwrap();
        let color_background =
            generate(Some(ColorPlace::Background), ColorType::Id("bright-blue")).unwrap();
        assert_eq!("\x1b[38;5;6m", color_foreground);
        assert_eq!("\x1b[48;5;12m", color_background);
    }
}
