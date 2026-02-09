# WelosaII

WelosaII is a creative, real-time animation app
It renders generative visuals, provides a live control UI, and can map animations to an LED grid over DDP (Only WLED Support for now).

**Project status:** early beta. Expect breaking changes, missing features, and rough edges.

## Highlights
- Multiple built-in animation modes 
- Master clock with BPM control and optional Ableton Link sync
- Live UI for tweaking settings and previewing output
- LED grid mapping with DDP output support
- ~~Preset saving for animator and grid settings~~ COMMING SOON

## Getting Started
### Prerequisites
- Rust toolchain (latest stable)

### Run
```bash
cargo run
```

## Controls
- `P`: presentation mode (hide UI)
- `E`: edit mode (show UI)


## Contributing
Thanks for helping! This project is in early beta, so contributions are especially welcome.

### How to contribute
1. Create a branch for your change.
2. Keep changes focused and small.
3. Use `cargo fmt` and `cargo clippy` when possible.
4. Open a PR with a clear description and screenshots/videos if the change is visual.

### Roadmap
* Shader Support
* Midi Support
* Art-Net Support
* Proper and less clutched Gui

