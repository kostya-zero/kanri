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

