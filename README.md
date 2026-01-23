# Humanboard

A desktop productivity app for macOS.

## Installation

### Homebrew (recommended)

```bash
brew tap humancorp-humancorp/humanboard
brew install --cask --no-quarantine humanboard
```

### Manual Download

1. Download the latest `.dmg` from [Releases](https://github.com/humancorp-humancorp/humanboard/releases)
2. Open the DMG and drag Humanboard to Applications
3. First launch: Right-click → Open → Open (to bypass Gatekeeper)

## Building from Source

Requires [Rust](https://rustup.rs/).

```bash
git clone https://github.com/humancorp-humancorp/humanboard.git
cd humanboard
cargo build --release
./build-app.sh
```

The app bundle will be at `Humanboard.app`.

## License

See [LICENSE](LICENSE).
