# Dactyl-RS

A simple blinky example for the nRF52840 microcontroller using the Embassy async framework.

## Overview

This project demonstrates a basic LED blinking example on the nRF52840 SoC using Rust and the Embassy async framework. The example is based on the [Embassy nRF52840 blinky example](https://github.com/embassy-rs/embassy/blob/main/examples/nrf52840/src/bin/blinky.rs).

The LED on pin P1_10 (see [Pinout](https://github.com/joric/nrfmicro/wiki/Pinout)) blinks every 300 milliseconds, providing a simple "Hello World" equivalent for embedded Rust development.

### Features

- **Simple LED Control**: Blinks an LED connected to pin P1_10
- **Async**: Built with Embassy async framework for efficient power management
- **Real-time Logging**: defmt-based logging via RTT for debugging
- **Embedded Rust**: Demonstrates `#![no_std]` embedded development

### Hardware Support

- **Target**: nRF52840 microcontroller
- **Architecture**: ARM Cortex-M4F (thumbv7em-none-eabihf)
- **LED**: Connected to pin P1_10
- **Debug Interface**: SWD with RTT logging

## Quick Start

### Building the Firmware

The project uses standard Cargo for building. Build the firmware with:

```bash
cargo build --target thumbv7em-none-eabihf
```

### Running the Example

You can run the blinky example directly using probe-rs:

```bash
probe-rs run --chip nRF52840_xxAA target/thumbv7em-none-eabihf/debug/dactyl-rs
```

Alternatively, use the VS Code task by pressing **F5** or **Ctrl+Shift+P** → "Tasks: Run Task" → "probe-rs run main".

### Expected Behavior

When the firmware is running, you should see:
1. The LED on pin P1_10 blinking every 300ms
2. Debug output via RTT showing the application start

## Development

### Prerequisites

1. **Rust Toolchain**:
   ```bash
   rustup target add thumbv7em-none-eabihf
   rustup component add llvm-tools-preview
   ```

2. **Development Tools**:
   ```bash
   cargo install flip-link
   cargo install cargo-binutils
   cargo install probe-rs --features cli
   ```

3. **Hardware**: 
   - nRF52840 development board (or compatible)
   - LED connected to pin P1_10 (or use onboard LED if available)
   - Debug probe (e.g., Raspberry Pi Debug Probe) for development and debugging

### Project Structure

The project is a simple single-binary embedded application:

```
src/
└── main.rs          # Main blinky application
```

### Building

Build the project:
```bash
cargo build --target thumbv7em-none-eabihf
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
2. Select **"probe-rs run main"** - Flash and run the blinky firmware

#### Command Line Debugging

Flash and debug the blinky example:
```bash
probe-rs run --chip nRF52840_xxAA target/thumbv7em-none-eabihf/debug/dactyl-rs
```

#### Expected Debug Output

When running, you should see defmt logs like:
```
0.000000 [INFO ] Blinky example starting...
0.000000 [INFO ] LED initialized on pin P1_10
0.300000 [DEBUG] LED ON
0.600000 [DEBUG] LED OFF
0.900000 [DEBUG] LED ON
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
- **`.vscode/tasks.json`**: Build and run tasks
- **`Probe.toml`**: probe-rs RTT and debugging configuration
- **`.cargo/config.toml`**: Cargo environment variables and target settings

### Contributing

1. Follow Rust formatting: `cargo fmt`
2. Check for issues: `cargo clippy`
3. Test the firmware on hardware
4. Update documentation for any API changes

## Hardware Setup

This example expects an LED connected to pin P1_10 of the nRFMicro. If your board uses a different pin, modify the pin assignment in `src/main.rs`:

```rust
let mut led = Output::new(p.PX_XX, Level::Low, OutputDrive::Standard); // Change PX_XX to your LED pin
```

### Continuous Integration

The project includes GitHub Actions workflow that:
- Builds the blinky firmware
- Runs code quality checks
- Provides downloadable artifacts for releases

## License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
