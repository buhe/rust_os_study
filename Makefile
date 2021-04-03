DOCKER_NAME ?= dinghao188/rcore-tutorial

docker:
	docker run --rm -it --mount type=bind,source=$(shell pwd),destination=/mnt ${DOCKER_NAME}
TARGET := riscv64gc-unknown-none-elf

env:
	(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(TARGET)
	cargo install cargo-binutils
	rustup component add rust-src
	rustup component add llvm-tools-preview
