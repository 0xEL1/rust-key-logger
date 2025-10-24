# Windows Input Logger

A low-level keyboard and mouse event logger for Windows systems, written in Rust using the Windows API.

## Overview

This application uses Windows hooks and  operates at the system level using `WH_KEYBOARD_LL` and `WH_MOUSE_LL` hooks to intercept input events before they reach applications.

## Features

- **Keyboard Event Logging**: Captures key presses, releases, and system key events
- **Mouse Event Logging**: Records mouse movements, button clicks, and wheel events
- **Timestamped Logs**: All events are logged with precise timestamps
- **Separate Log Files**: Keyboard and mouse events are stored in separate files for easier analysis

## Requirements

- Windows Operating System
- Rust toolchain (stable)
- `winapi` crate dependency

## Building

Add the following dependency to your `Cargo.toml`:

```toml
[dependencies]
winapi = { version = "0.3", features = ["winuser"] }
```

Build the project:

```bash
cargo build --release
```

## Usage

Run the compiled executable:

```bash
cargo run --release
```

The application will start logging immediately and create two files:
- `kdb_log.txt` - Keyboard events
- `ms_log.txt` - Mouse events

To stop logging, terminate the process (Ctrl+C or close the window).


## Disclaimer

Use this software responsibly and only on systems you own or have explicit permission to monitor. The developers assume no liability for misuse of this software.

---

Happy hacking :D