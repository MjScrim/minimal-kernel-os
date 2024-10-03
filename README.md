
To run this project, you need to have the [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) and [QEMU](https://www.qemu.org/download/) installed.

In the project directory
```bash
cargo install
rustup target add thumbv7em-none-eabihf
cargo build --target thumbv7em-none-eabihf
cargo build --target x86_64-scrim_os.json
cargo install bootimage
cargo bootimage
cargo run 
