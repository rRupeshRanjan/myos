## build with
- cargo build --target thumbv7em-none-eabihf <-- old>
- cargo bootimage
- qemu-system-x86_64 -drive format=raw,file=target/x86_64-myos/debug/bootimage-myos.bin
- Above can be replaced with "cargo run" instead after adding runner in .cargo/config.toml
- cargo run -vnc :1 (for vnc output to post 5901)
- cargo test -vnc :1

## extra commands needed
- rustup target add thumbv7em-none-eabihf
- rustup component add rust-src
- rustup component add llvm-tools-preview
- cargo install bootimage

## sample output:
<img width="1129" height="790" alt="image" src="https://github.com/user-attachments/assets/9d9840a4-755f-4206-8445-5490e5d2ac1d" />
<img width="720" height="463" alt="image" src="https://github.com/user-attachments/assets/210ac029-515d-48a6-946c-bff0defe6172" />
<img width="719" height="454" alt="image" src="https://github.com/user-attachments/assets/71588b3a-d4f8-4e10-b178-bbf5529eefbf" />
