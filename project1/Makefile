# 项目根目录的 Makefile
LOG ?= INFO

run:
	# 通过 RUSTFLAGS 向 rustc 传递 --cfg 参数
	cd os && RUSTFLAGS="--cfg log_level=\"$(LOG)\"" cargo build --release
	# 转换为 bin 文件
	rust-objcopy --binary-architecture=riscv64 os/target/riscv64gc-unknown-none-elf/release/os --strip-all -O binary os/target/riscv64gc-unknown-none-elf/release/os.bin
	# 运行 QEMU
	qemu-system-riscv64 -machine virt -nographic -bios ./bootloader/rustsbi-qemu.bin -device loader,file=./os/target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000

clean:
	cd os && cargo clean
	rm -f os/target/riscv64gc-unknown-none-elf/release/os.bin