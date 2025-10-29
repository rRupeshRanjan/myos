build with

- cargo build --target thumbv7em-none-eabihf <-- old>
- cargo bootimage
- qemu-system-x86_64 -drive format=raw,file=target/x86_64-myos/debug/bootimage-myos.bin
- Above can be replaced with "cargo run" instead after adding runner in .cargo/config.toml

extra commands needed

- rustup target add thumbv7em-none-eabihf
- rustup component add rust-src
- rustup component add llvm-tools-preview
- cargo install bootimage
