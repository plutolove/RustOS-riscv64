export LOG_LEVEL=INFO
cargo build
rust-objcopy --binary-architecture=riscv64 target/riscv64gc-unknown-none-elf/debug/os --strip-all -O binary target/riscv64gc-unknown-none-elf/debug/os.bin

qemu-system-riscv64 -machine virt -nographic -bios ./bootloader/rustsbi-qemu.bin -device loader,file=target/riscv64gc-unknown-none-elf/debug/os.bin,addr=0x80200000

sudo rmmod btusb
sudo rmmod btintel

sudo modprobe btintel
sudo modprobe btusb
