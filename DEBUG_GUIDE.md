# Debug Configuration Guide

This project is now configured for proper defmt/RTT debugging with VS Code and probe-rs.

## ✅ What's Configured

### 1. **RTT Output in VS Code Debugging**
- RTT channel 0 configured for defmt output
- Timestamps and location information enabled
- Auto-detects defmt format

### 2. **Environment Variables**
- `DEFMT_LOG=debug` set in `.cargo/config.toml`
- Enables all debug-level defmt logs

### 3. **VS Code Integration**
- Launch configuration automatically builds and flashes
- RTT terminal tabs open automatically
- Debug console shows probe-rs logs

## 🚀 How to Use

### **Option 1: VS Code Debugging (RTT in Terminal)**
1. Press **F5** or **Run > Start Debugging**
2. VS Code will:
   - Build the project automatically
   - Flash the firmware
   - Start debugging session
   - Open RTT terminal tab showing defmt logs

### **Option 2: Task with RTT Output**
1. **Ctrl+Shift+P** → "Tasks: Run Task"
2. Select **"probe-rs run with RTT"**
3. See real-time defmt logs in terminal

### **Option 3: Command Line**
```bash
probe-rs run --chip nRF52840_xxAA target/thumbv7em-none-eabihf/debug/dactyl-rs
```

## 📋 Expected Output

When running, you should see defmt logs like:
```
0.000000 [INFO ] === Dactyl keyboard firmware starting === (dactyl_rs dactyl-rs/src/main.rs:37)
0.000000 [INFO ] Defmt logging system initialized (dactyl_rs dactyl-rs/src/main.rs:38)
0.000000 [INFO ] Dactyl keyboard starting... (dactyl_rs dactyl-rs/src/main.rs:40)
0.000000 [INFO ] Enabling ext hfosc... (dactyl_rs dactyl-rs/src/main.rs:45)
0.000427 [INFO ] External HFOSC enabled successfully (dactyl_rs dactyl-rs/src/main.rs:48)
0.123456 [INFO ] Key pressed at (2, 3): 65 (dactyl_rs dactyl-rs/src/matrix.rs:42)
```

## 🔧 Troubleshooting

### **No RTT Output in VS Code**
1. Check that RTT terminal tab opened during debug session
2. Look for "probe-rs-rtt-0" or similar terminal tab
3. Verify `rttEnabled: true` in launch.json

### **Build Errors**
1. Ensure probe-rs is installed: `cargo install probe-rs --features cli`
2. Check that the debug probe is connected: `probe-rs list`

### **defmt Logs Filtered**
1. Check DEFMT_LOG environment variable is set to "debug"
2. Verify defmt-rtt dependency in Cargo.toml

## 📁 Configuration Files

- **`.vscode/launch.json`**: Debug configuration with RTT
- **`.vscode/tasks.json`**: Build and run tasks
- **`Probe.toml`**: probe-rs RTT configuration
- **`.cargo/config.toml`**: Cargo environment variables
