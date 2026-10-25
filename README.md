# Simple Calendar

A simple desktop calendar application built with Rust and Freya.

<img width="592" height="520" src="https://github.com/user-attachments/assets/60afb2de-7369-44c6-944c-3da2c16e3d83" />

# Installation

## One-liners
Each one downloads the latest release.

**Linux** (Flatpak, recommended):
```bash
d=$(mktemp -d) && curl -fL "$(curl -fs https://api.github.com/repos/andunieee/simple-calendar/releases/latest | grep -o 'https://[^"]*\.flatpak')" -o "$d/simple-calendar.flatpak" && flatpak install --user -y "$d/simple-calendar.flatpak"; rm -rf "$d"
```

**macOS** (Apple Silicon):
```
curl -fL "$(curl -s https://api.github.com/repos/andunieee/simple-calendar/releases/latest | grep -o 'https://[^"]*macos-arm64[^"]*\.zip')" -o /tmp/simple-calendar.zip && ditto -x -k /tmp/simple-calendar.zip /Applications
```

**Windows** (PowerShell), runs the installer:
```
$u=((irm https://api.github.com/repos/andunieee/simple-calendar/releases/latest).assets.browser_download_url -match 'setup')[0]; iwr $u -OutFile $env:TEMP\sc-setup.exe; & $env:TEMP\sc-setup.exe
```

**Anywhere with Rust**: `cargo install simple-calendar`

## Build from source

```bash
# Clone the repository
git clone https://github.com/andunieee/simple-calendar.git
cd simple-calendar

# Build release binary
cargo build --release

# Run
cargo run --release
```

The binary will be available at `target/release/simple-calendar`.

## Download

Every `v*` tag publishes a `.flatpak` and a `.tar.gz` for Linux, a setup `.exe` and a `.zip` for Windows and a `.zip` with `simple-calendar.app` for Apple Silicon macOS on the [releases page](https://github.com/andunieee/simple-calendar/releases).

## License

This project is licensed under the MIT License.
