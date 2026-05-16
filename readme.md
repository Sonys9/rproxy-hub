# RProxy Hub

Rust Proxy Hub

⚡️ Blazing fast traffic forwarder using proxies ⚡️

*🛠 Currently it's in developing 🛠*

![App Logo](assets/app.png)

# Why?
- 🔥 **Blazingly fast** — Built in pure Rust with zero garbage collection. It just flies.
- 🦀 **Guaranteed stability** — Compiler-enforced memory safety means no random segfaults or runtime crashes.
- ⚡ **Async under the hood** — Driven by Tokio to handle thousands of concurrent proxy connections with near-zero latency.
- 🎨 **Clean CLI** — Features good interface and high customizable banner system

## 🗺️ Banner Placeholders Reference

You can fully customize the application startup banner using dynamic template tags. The parser will automatically replace these tags with live server configurations and ANSI colors.

### 🧩 System Variables

Use these placeholders to display live runtime information from your configuration:


| Placeholder | Description | Example Output |
| :--- | :--- | :--- |
| `%app_version%` | Application version parsed from `Cargo.toml` | `0.1.0` |
| `%listen_ip%` | The IP and port the server is currently binding to | `127.0.0.1:0` |
| `%forward_to%` | The destination target IP and port for routing | `12.67.12.8:9822` |
| `%proxies_path%` | System path pointing to your active proxy list file | `proxies.txt` |

---

### 🎨 Color & Styling Markers

You can style text and background using both standard named profiles and 24-bit TrueColor (RGB).

#### 1. Text (Foreground) Styling
* **`%color_fg_NAME%`** — Set text color by its standard name.
  * *Available names (defined in `colors.rs`):* `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`, `black`, etc.
* **`%color_fg_rgb_R_G_B%`** — Set raw 24-bit TrueColor text color. Replace `R`, `G`, `B` with values from `0` to `255`.
  * *Example:* `%color_fg_rgb_255_165_0%` creates orange text.
* **`%color_fg_reset%`** — Reset text color back to your terminal's default layout.

#### 2. Background Styling
* **`%color_bg_NAME%`** — Set background color by its standard name.
* **`%color_bg_rgb_R_G_B%`** — Set raw 24-bit TrueColor background layout.
  * *Example:* `%color_bg_rgb_40_40_40%` creates a dark gray background layer.
* **`%color_bg_reset%`** — Reset background color back to your terminal's default layout.

---

# Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

# Code of Conduct

This project adheres to a [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

# Security

For security concerns, please refer to our [Security Policy](SECURITY.md).