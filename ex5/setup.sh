rustup target add riscv64imac-unknown-none-elf
cargo install cargo-binutils
rustup component add llvm-tools-preview
cargo install rcore-fs-fuse --git https://github.com/rcore-os/rcore-fs
