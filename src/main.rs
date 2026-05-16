use crate::colors::{
    ColorPlace::{Background, Foreground},
    ColorType::{Id, Rgb, Style},
    RgbColor, generate,
};
use clap::Parser;
use log::info;
use regex::{Captures, Regex};
use std::{net::SocketAddr, path::PathBuf};
mod colors;

fn parse_byte(byte: Option<&&str>) -> u8 {
    byte.and_then(|r| r.parse::<u8>().ok()).unwrap_or(255)
}

fn parse_caps(caps: &Captures) -> String {
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

fn display_banner(listen_ip: SocketAddr, forward_to: SocketAddr, proxies_path: PathBuf) {
    // Banner available placeholders:
    // 1. %app_version% - App version from Cargo.toml (example: 0.1.0)
    // 2. %listen_ip% - Listen ip (example: 127.0.0.1:0)
    // 3. %forward_to% - Forward to ip (example: 12.67.12.8:9822)
    // 4. %proxies_path% - Path to a file with proxies (example: proxies.txt)
    // 5. Colors:
    // 5.1. %color_fg_rgb_R_G_B% - Foreground (text) color marker where R, G and B are numbers 0-255
    // 5.2. %color_fg_NAME% - Foreground (text) color marker where NAME is string color name (example: red, you can get available in COLORS from colors.rs)
    // 5.3. %color_bg_rgb_R_G_B% - Same but for background
    // 5.4. %color_bg_NAME% - Same but for background
    // 5.5. %color_STYLE% - Foreground (text) color style (reset, bold, dim, italic, underline, blink, invert)
    let uncolored_banner = include_str!("../banner.txt")
        .replace("%app_version%", env!("CARGO_PKG_VERSION"))
        .replace("%listen_ip%", &listen_ip.to_string())
        .replace("%forward_to%", &forward_to.to_string())
        .replace("%proxies_path%", &proxies_path.to_string_lossy());
    let colors_regex = Regex::new(r"%color_[^%]*%").expect("Failed to generate regex");
    let banner = colors_regex.replace_all(&uncolored_banner, |caps: &Captures| parse_caps(caps));
    println!("{}", banner);
}

#[derive(clap::Parser, Debug)]
#[command(
    name = "rproxy-hub",
    about = "Proxy forwarder",
    after_help = "Note: listen ip is optional, defaults to 127.0.0.1:0 (:0 means random port)\n\nExample:\n\tapp 12.67.12.8:9822 proxies.txt"
)]
struct Args {
    /// Forward to IP address
    #[arg(value_name = "FORWARD TO IP")]
    forward_to: SocketAddr,

    /// Path to proxies file
    #[arg(value_name = "PATH TO PROXIES")]
    proxies_path: PathBuf,

    /// Listen IP address
    #[arg(value_name = "LISTEN IP", default_value = "127.0.0.1:0")]
    listen_ip: SocketAddr,

    /// Do not print the banner
    #[arg(short, long)]
    silent: bool,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();
    if !args.silent {
        display_banner(args.listen_ip, args.forward_to, args.proxies_path)
    };
    info!("Waiting for packets...");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_parser() {
        let result = parse_byte(Some(&"123"));
        assert_eq!(result, 123);
    }

    #[test]
    fn test_byte_parser_bad_number() {
        let result = parse_byte(Some(&"bad_number"));
        assert_eq!(result, 255);
    }

    #[test]
    fn test_color_parser() {
        let colors_regex = Regex::new(r"%color_[^%]*%").expect("Failed to generate regex");
        let test_banner = "%color_fg_rgb_255_165_0%\n%color_reset%\n%color_fg_cyan%";
        let banner = colors_regex
            .replace_all(test_banner, |caps: &Captures| parse_caps(caps))
            .to_string();
        assert_eq!(
            banner,
            format!(
                "{}\n{}\n{}",
                colors::generate(
                    Some(Foreground),
                    Rgb(RgbColor {
                        r: 255,
                        g: 165,
                        b: 0
                    })
                )
                .unwrap(),
                colors::generate(Some(Foreground), Style(colors::Style { name: "reset" })).unwrap(),
                colors::generate(Some(Foreground), Id("cyan")).unwrap()
            )
        );
    }
}
