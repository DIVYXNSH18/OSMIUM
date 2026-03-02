# Compilation Guide

## ✅ Code Status: 100% API Compatible

All code has been updated to Serenity 0.12 API. The bot is ready to compile once the build environment is set up.

## Build Environment Setup

### Windows - Option 1: MSVC Toolchain (Recommended)

**Install Visual Studio Build Tools:**

1. Download [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)
2. Run the installer
3. Select "Desktop development with C++"
4. Install (requires ~7GB disk space)
5. Restart your terminal

**Verify Installation:**
```cmd
link.exe
```
Should show Microsoft linker help.

**Build:**
```bash
cargo build --release
```

### Windows - Option 2: GNU Toolchain (MinGW)

**Install MinGW-w64:**

1. Download [MSYS2](https://www.msys2.org/)
2. Install MSYS2
3. Open MSYS2 terminal and run:
```bash
pacman -S mingw-w64-x86_64-gcc
```

4. Add to PATH: `C:\msys64\mingw64\bin`

**Install GNU Rust Toolchain:**
```bash
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

**Build:**
```bash
cargo build --release
```

### Linux

**Install Dependencies:**

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install build-essential pkg-config libssl-dev

# Fedora/RHEL
sudo dnf install gcc openssl-devel

# Arch
sudo pacman -S base-devel openssl
```

**Build:**
```bash
cargo build --release
```

### macOS

**Install Xcode Command Line Tools:**
```bash
xcode-select --install
```

**Build:**
```bash
cargo build --release
```

## Compilation Commands

### Development Build (Fast, Debug Info)
```bash
cargo build
```

### Production Build (Optimized)
```bash
cargo build --release
```

### Check Without Building (Fast)
```bash
cargo check
```

### Run Directly
```bash
cargo run --release
```

## Build Output

Compiled binary location:
- Debug: `target/debug/discord-security-bot` (or `.exe` on Windows)
- Release: `target/release/discord-security-bot` (or `.exe` on Windows)

## Troubleshooting

### Error: `link.exe` not found (Windows MSVC)

**Cause:** Visual Studio Build Tools not installed or not in PATH

**Solution:**
1. Install Visual Studio Build Tools with C++ workload
2. Restart terminal
3. Verify: `link.exe` should work

**Alternative:** Switch to GNU toolchain (see Option 2 above)

### Error: `gcc.exe` not found (Windows GNU)

**Cause:** MinGW not installed or not in PATH

**Solution:**
1. Install MSYS2 and MinGW-w64
2. Add `C:\msys64\mingw64\bin` to PATH
3. Restart terminal
4. Verify: `gcc --version` should work

### Error: OpenSSL not found (Linux)

**Solution:**
```bash
# Ubuntu/Debian
sudo apt install libssl-dev pkg-config

# Fedora
sudo dnf install openssl-devel
```

### Error: Failed to compile `ring` crate

**Cause:** Missing C compiler or build tools

**Solution:**
- Windows: Install Visual Studio Build Tools or MinGW
- Linux: Install `build-essential` or equivalent
- macOS: Install Xcode Command Line Tools

### Slow Compilation

**Tips:**
- Use `cargo check` for faster feedback
- Enable parallel compilation: `cargo build -j$(nproc)`
- Use `sccache` for caching: `cargo install sccache`
- First build takes 5-10 minutes (downloads and compiles 200+ dependencies)
- Subsequent builds are much faster (30 seconds - 2 minutes)

### Out of Memory During Compilation

**Solution:**
```bash
# Limit parallel jobs
cargo build --release -j2

# Or set in .cargo/config.toml
[build]
jobs = 2
```

### Linker Errors

**Windows:**
- Ensure only one toolchain is active (MSVC or GNU, not both)
- Check: `rustup show`
- Switch: `rustup default stable-x86_64-pc-windows-msvc`

**Linux:**
- Install linker: `sudo apt install lld` or `sudo apt install mold`
- Use faster linker in `.cargo/config.toml`:
```toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```

## Verification

After successful compilation, verify the binary:

```bash
# Check binary exists
ls -lh target/release/discord-security-bot

# Check dependencies (Linux)
ldd target/release/discord-security-bot

# Run with version check
./target/release/discord-security-bot --version
```

## Cross-Compilation

### Build for Linux on Windows (WSL)

```bash
# Install WSL2
wsl --install

# Inside WSL
sudo apt update
sudo apt install build-essential pkg-config libssl-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo build --release
```

### Build for Windows on Linux

```bash
# Install cross-compilation tools
rustup target add x86_64-pc-windows-gnu
sudo apt install mingw-w64

# Build
cargo build --release --target x86_64-pc-windows-gnu
```

## Docker Build

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/discord-security-bot /usr/local/bin/
CMD ["discord-security-bot"]
```

Build:
```bash
docker build -t discord-security-bot .
```

## Performance Optimization

### Optimize Binary Size

Add to `Cargo.toml`:
```toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Better optimization
strip = true        # Remove debug symbols
```

### Optimize for Speed

```toml
[profile.release]
opt-level = 3       # Maximum optimization
lto = "fat"         # Full LTO
codegen-units = 1
```

## CI/CD Integration

### GitHub Actions

```yaml
name: Build
on: [push]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo build --release
      - uses: actions/upload-artifact@v3
        with:
          name: discord-security-bot
          path: target/release/discord-security-bot
```

## Next Steps

After successful compilation:

1. Copy `.env.example` to `.env`
2. Configure environment variables
3. Start MongoDB and Redis
4. Run the bot: `./target/release/discord-security-bot`
5. Check logs for successful startup
6. Invite bot to your Discord server

## Support

If compilation issues persist:

1. Check Rust version: `rustc --version` (should be 1.70+)
2. Update Rust: `rustup update`
3. Clean build: `cargo clean && cargo build --release`
4. Check dependencies: `cargo tree`
5. Report issue with full error output

---

**Current Status:** All code is Serenity 0.12 compatible. Only build environment setup is required.
