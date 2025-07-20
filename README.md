# Dactyl-RS

A Rust firmware implementation for the Dactyl split keyboard using the nRF52840 microcontroller.

## Overview

This project provides firmware for a wireless split keyboard based on the Dactyl design. The firmware is built with Rust using Embassy async framework and targets the nRF52840 SoC for Bluetooth Low Energy connectivity.

### Features

- **Split Design**: Separate firmware for left and right keyboard halves
- **Wireless**: Bluetooth Low Energy connectivity via nRF52840
- **Async**: Built with Embassy async framework for efficient power management
- **USB Support**: USB HID when connected via cable
- **Real-time Logging**: defmt-based logging via RTT for debugging

### Hardware Support

- **Target**: nRF52840 microcontroller
- **Architecture**: ARM Cortex-M4F (thumbv7em-none-eabihf)
- **Connectivity**: Bluetooth LE, USB
- **Debug Interface**: SWD with RTT logging

## Quick Start

### Building the Firmware

The project uses `cargo-make` for build automation. Install it first:

```bash
cargo install cargo-make
```

Build both firmware binaries and generate UF2 files for flashing:

```bash
cargo make uf2
```

This will generate:
- `left.uf2` - Firmware for the left keyboard half
- `right.uf2` - Firmware for the right keyboard half

### Flashing

1. Put your keyboard half into bootloader mode
2. Copy the corresponding UF2 file to the USB drive that appears
3. The firmware will be automatically flashed and the keyboard will restart

## Development

### Prerequisites

1. **Rust Toolchain**:
   ```bash
   rustup target add thumbv7em-none-eabihf
   rustup component add llvm-tools-preview
   ```

2. **Development Tools**:
   ```bash
   cargo install cargo-make
   cargo install flip-link
   cargo install cargo-binutils
   cargo install cargo-hex-to-uf2
   cargo install probe-rs --features cli
   ```

3. **Hardware**: Debug probe (e.g., J-Link, ST-Link) for development and debugging

### Project Structure

```
src/
├── left.rs          # Left keyboard half firmware entry point
├── right.rs         # Right keyboard half firmware entry point
├── lib.rs           # Shared library code
├── matrix.rs        # Key matrix scanning
├── layout.rs        # Key layout and mapping
├── keycodes.rs      # HID keycodes
└── usb.rs           # USB HID implementation
```

### Building Individual Halves

Build just the left half:
```bash
cargo build --bin left --target thumbv7em-none-eabihf
```

Build just the right half:
```bash
cargo build --bin right --target thumbv7em-none-eabihf
```

### Debugging

This project is configured for comprehensive debugging with defmt/RTT logging via probe-rs.

#### VS Code Integration

**Option 1: Debug with VS Code (Recommended)**
1. Connect your debug probe to the keyboard half
2. Press **F5** or **Run > Start Debugging**
3. VS Code will:
   - Build the project automatically
   - Flash the firmware to the connected half
   - Start debugging session
   - Open RTT terminal showing real-time logs

**Option 2: Run Tasks**
1. **Ctrl+Shift+P** → "Tasks: Run Task"
2. Select either:
   - **"probe-rs run left"** - Flash and run left half firmware
   - **"probe-rs run right"** - Flash and run right half firmware

#### Command Line Debugging

Flash and debug the left half:
```bash
probe-rs run --chip nRF52840_xxAA target/thumbv7em-none-eabihf/debug/left
```

Flash and debug the right half:
```bash
probe-rs run --chip nRF52840_xxAA target/thumbv7em-none-eabihf/debug/right
```

#### Expected Debug Output

When running, you should see defmt logs like:
```
0.000000 [INFO ] === Dactyl keyboard firmware starting === (left/src/left.rs:37)
0.000000 [INFO ] Defmt logging system initialized (left/src/left.rs:38)
0.000000 [INFO ] Left keyboard half starting... (left/src/left.rs:40)
0.000000 [INFO ] Enabling ext hfosc... (left/src/left.rs:45)
0.000427 [INFO ] External HFOSC enabled successfully (left/src/left.rs:48)
0.123456 [INFO ] Key pressed at (2, 3): 65 (dactyl_rs/src/matrix.rs:42)
```

#### Troubleshooting

**No RTT Output**:
1. Verify debug probe connection: `probe-rs list`
2. Check RTT terminal tab opened in VS Code
3. Ensure `DEFMT_LOG=debug` environment variable is set

**Build Errors**:
1. Ensure all required tools are installed
2. Check Rust target is installed: `rustup target list --installed`
3. Verify probe-rs installation: `probe-rs --version`

**Connection Issues**:
1. Check debug probe is recognized: `probe-rs list`
2. Verify chip detection: `probe-rs info --chip nRF52840_xxAA`
3. Try different SWD clock speeds in Probe.toml

### Configuration Files

- **`.vscode/launch.json`**: VS Code debug configuration with RTT support
- **`.vscode/tasks.json`**: Build and run tasks for both halves
- **`Probe.toml`**: probe-rs RTT and debugging configuration
- **`.cargo/config.toml`**: Cargo environment variables and target settings
- **`Makefile.toml`**: cargo-make build automation tasks

### Contributing

1. Follow Rust formatting: `cargo fmt`
2. Check for issues: `cargo clippy`
3. Test both firmware halves
4. Update documentation for any API changes

### Continuous Integration

The project includes GitHub Actions workflow that:
- Builds firmware for both keyboard halves
- Generates UF2 files for easy flashing
- Provides downloadable artifacts for releases

## License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
