# Building Kanri

This guide provides instructions for building Kanri manually from source.

### Prerequisites

Before you begin, ensure you have the following:

- The latest version of the Rust toolchain.
- A C/C++ compiler:
  - **Windows**: The latest version of _Visual Studio Build Tools 2022_ (including the Windows SDK).
  - **Linux** and **macOS**: The latest version of _GCC_ or _Clang_.

If you have already installed the Rust toolchain and a C/C++ compiler, you can proceed to [Step 2: Download Source Code](#step-2-download-source-code).

### Step 1: Install Required Tools

If you don't have Rust installed, visit the [official Rust installation page](https://www.rust-lang.org/tools/install) and follow the instructions.

After installation, verify that the Rust toolchain (e.g., `rustc --version`, `cargo --version`) and your C/C++ compiler are correctly installed and accessible from your terminal.

### Step 2: Download Source Code

You can download the Kanri source code using `git` or by downloading a ZIP archive.

**Using Git (Recommended):**
```shell
git clone https://github.com/kostya-zero/kanri.git
cd kanri 
```

**Downloading ZIP Archive:**
1.  Go to the [Kanri GitHub repository](https://github.com/kostya-zero/kanri).
2.  Click on "Code" -> "Download ZIP".
3.  Extract the contents of the ZIP file to your desired location.

### Step 3: Build Kanri

Navigate to the root directory of the Kanri source code in your terminal.

To build Kanri, run:
```shell
cargo build
```

For an optimized release build, run:
```shell
cargo build --release
```

The compiled binary will be located in `target/debug/` for a regular build or `target/release/` for a release build.

### Step 4: Run checks

Before submitting changes or creating a release, run:

```shell
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

CI also runs the test suite with `cargo nextest run`.

### Advanced Configuration & Tweaks

These optional tweaks can improve compilation speed or runtime performance. 
For persistent settings across sessions, it's recommended to use a `.cargo/config.toml` file.

#### Using `.cargo/config.toml` for Persistent Settings

Create a file named `config.toml` inside a `.cargo` directory at the root of the Kanri project (i.e., `kanri/.cargo/config.toml`). This file allows you to specify various build options, including linkers and compilation flags, which Cargo will automatically apply.

#### Tweak 1: Faster Linking with LLD

Using the LLD linker (from the LLVM project) can significantly speed up the linking phase of compilation.

**A. Install LLD:**

-   **Windows:**
    1.  Ensure LLVM tools are installed. This can often be selected as an individual component ("C++ Clang tools for Windows") in the Visual Studio Installer, or by downloading the official LLVM toolchain installer.
    2.  Verify that `lld-link.exe` is in your system's PATH.
-   **Linux:**
    1.  Install `lld` (and `clang` if not already present) using your package manager.
        *   Debian/Ubuntu: `sudo apt update && sudo apt install lld clang`
        *   Fedora: `sudo dnf install lld clang`
-   **macOS:**
    1.  Install LLVM via Homebrew: `brew install llvm`.
    2.  `clang` usually comes with Xcode Command Line Tools. If `lld` is not found by default, you might need to add LLVM's `bin` directory to your PATH or specify the full path to `ld.lld` in `rustflags`.

**B. Configure Cargo for LLD:**

Add the appropriate lines to your `kanri/.cargo/config.toml` file:

```toml
# .cargo/config.toml

# For Windows MSVC target
[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "link-arg=-fuse-ld=lld-link"]

# For Linux targets (e.g., x86_64-unknown-linux-gnu)
# Replace with your specific target if needed.
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

# For macOS targets (e.g., x86_64-apple-darwin or aarch64-apple-darwin)
# Replace with your specific target if needed.
[target.x86_64-apple-darwin]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[target.aarch64-apple-darwin]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```

#### Tweak 2: Native Compilation for Performance

This optimizes the build for the specific CPU architecture of the machine you are compiling on. 
This can potentially increase runtime performance but makes the resulting binary less portable (it will not run on machines with different CPU microarchitecture).

To enable native compilation, add the `-C target-cpu=native` flag to your `rustflags` in `kanri/.cargo/config.toml`.

**Option 1: Global Native Compilation (applies to all builds for this project)**
Add to the `[build]` section in `kanri/.cargo/config.toml`:
```toml
# .cargo/config.toml
[build]
rustflags = ["-C", "target-cpu=native"]
```

**Option 2: Target-Specific Native Compilation**
Add to a specific target section in `kanri/.cargo/config.toml` (e.g., for `x86_64-unknown-linux-gnu`):
```toml
# .cargo/config.toml
[target.x86_64-unknown-linux-gnu] # Replace with your actual target triple
rustflags = ["-C", "target-cpu=native"]
```

#### Combining Tweaks

You can combine multiple `rustflags` in your `.cargo/config.toml`. For example, to use LLD and enable native CPU optimization for a Linux target:

```toml
# .cargo/config.toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = [
    "-C", "link-arg=-fuse-ld=lld", # Use LLD
    "-C", "target-cpu=native"      # Optimize for native CPU
]
```
Ensure that `rustflags` is an array of strings, with each flag or flag-value pair as separate elements if they contain spaces or need to be distinct arguments for the compiler.
