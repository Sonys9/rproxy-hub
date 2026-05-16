use clap::Parser;
use log::error;
use regex::{Captures, Regex};
use std::{net::SocketAddr, path::PathBuf};
use crate::tcp::Tcp;
mod colors;
mod parsers;
mod tcp;
mod proxy;
mod udp;

fn display_banner(listen_ip: &str, forward_to: &str, proxies_path: &str, proxies_count: usize) {
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
        .replace("%listen_ip%", listen_ip)
        .replace("%forward_to%", forward_to)
        .replace("%proxies_path%", proxies_path)
        .replace("%proxies_count%", &proxies_count.to_string());
    let colors_regex = Regex::new(r"%color_[^%]*%").expect("Failed to generate regex");
    let banner = colors_regex.replace_all(&uncolored_banner, |caps: &Captures| {
        parsers::parse_caps(caps)
    });
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
    #[arg(
        short('l'),
        long("listen-ip"),
        value_name = "LISTEN IP",
        default_value = "127.0.0.1:0"
    )]
    listen_ip: SocketAddr,

    /// Do not print the banner
    #[arg(short, long)]
    silent: bool,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();
    let proxies = parsers::parse_proxies(&args.proxies_path)
        .await
        .expect("Failed to parse proxies");
    if proxies.is_empty() {
        return error!(
            "Proxy file is empty at {}!",
            args.proxies_path.to_string_lossy()
        );
    };
    if !args.silent {
        display_banner(
            &args.listen_ip.to_string(),
            &args.forward_to.to_string(),
            &args.proxies_path.to_string_lossy(),
            proxies.len(),
        );
    };
    Tcp {
        listen_ip: args.listen_ip,
        forward_to: args.forward_to,
        proxies,
    }
    .start_loop()
    .await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_parser() {
        let result = parsers::parse_byte(Some(&"123"));
        assert_eq!(result, 123);
    }

    #[test]
    fn test_byte_parser_bad_number() {
        let result = parsers::parse_byte(Some(&"bad_number"));
        assert_eq!(result, 255);
    }

    #[test]
    fn test_color_parser() {
        let colors_regex = Regex::new(r"%color_[^%]*%").expect("Failed to generate regex");
        let test_banner = "%color_fg_rgb_255_165_0%\n%color_reset%\n%color_fg_cyan%";
        let banner = colors_regex
            .replace_all(test_banner, |caps: &Captures| parsers::parse_caps(caps))
            .to_string();
        assert_eq!(
            banner,
            format!(
                "{}\n{}\n{}",
                colors::generate(
                    Some(colors::ColorPlace::Foreground),
                    colors::ColorType::Rgb(colors::RgbColor {
                        r: 255,
                        g: 165,
                        b: 0
                    })
                )
                .unwrap(),
                colors::generate(
                    Some(colors::ColorPlace::Foreground),
                    colors::ColorType::Style(colors::Style { name: "reset" })
                )
                .unwrap(),
                colors::generate(
                    Some(colors::ColorPlace::Foreground),
                    colors::ColorType::Id("cyan")
                )
                .unwrap()
            )
        );
    }
}
