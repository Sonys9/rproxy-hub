use std::path::PathBuf;

use crate::colors::{
    self,
    ColorPlace::{Background, Foreground},
    ColorType::{Id, Rgb, Style},
    RgbColor, generate,
};
use regex::Captures;

pub fn parse_byte(byte: Option<&&str>) -> u8 {
    byte.and_then(|r| r.parse::<u8>().ok()).unwrap_or(255)
}

pub fn parse_caps(caps: &Captures) -> String {
    let color_parts: Vec<&str> = caps[0][1..caps[0].len() - 1].split("_").collect();
    let &place_str = color_parts.get(1).unwrap_or(&"");
    let place = match place_str {
        "fg" => Some(Foreground),
        "bg" => Some(Background),
        _ => None,
    };
    let color_type = color_parts.get(2).unwrap_or(&"");
    match *color_type {
        "rgb" => generate(
            place,
            Rgb(RgbColor {
                r: parse_byte(color_parts.get(3)),
                g: parse_byte(color_parts.get(4)),
                b: parse_byte(color_parts.get(5)),
            }),
        )
        .unwrap_or("".to_string()),
        _ if place.is_some() => {
            generate(place, Id(color_parts.get(2).unwrap_or(&""))).unwrap_or_default()
        }
        _ => generate(
            None,
            Style(colors::Style {
                name: color_parts.get(1).unwrap_or(&""),
            }),
        )
        .unwrap_or_default(),
    }
}

pub async fn parse_proxies(file: &PathBuf) -> Result<Vec<String>, String> {
    if !file.exists() {
        return Err("Failed to load proxies: file not found".to_string());
    };
    let proxies = tokio::fs::read_to_string(file)
        .await
        .map_err(|e| format!("Failed to read file: {}", e))?
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();
    Ok(proxies)
}