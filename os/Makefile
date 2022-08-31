PROJ:=os
ARCH:=riscv64gc-unknown-none-elf

# For building rust
CARGO:=cargo
FMT:=fmt
BUILD:=build
BUILD_TYPE:=release
TARGET_DIR:=target/$(ARCH)/$(BUILD_TYPE)

# For cutting EOF file to get kernel.bin
OBJCOPY:=rust-objcopy
OBJCOPY_FLAG:=--strip-all

# For running on qemu
BOOT_DIR:=../bootloader
QEMU_SYS:=qemu-system-riscv64

QEMU_USER:=qemu-riscv64
QEMU_RUSTSBI:=rustsbi-qemu.bin

KERNEL:=$(TARGET_DIR)/$(PROJ)
KERNEL_BIN:=$(TARGET_DIR)/$(PROJ).bin
KERNEL_ENTRY_PA:=0x80200000
BOOTLOADER:=$(BOOT_DIR)/$(QEMU_RUSTSBI)

default: clean format build cut run

.PHONY: build cut run clean format

build: $(PROJ)

$(PROJ):
	@$(CARGO) $(BUILD) --$(BUILD_TYPE) --target $(ARCH)

cut: build $(PROJ).bin

$(PROJ).bin:
	@$(OBJCOPY) $(OBJCOPY_FLAG) $(KERNEL) -O binary $(KERNEL_BIN)

run: cut
	@$(QEMU_SYS) \
        -machine virt \
        -nographic \
        -bios $(BOOTLOADER) \
        -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)

clean:
	@rm -rf target/

format:
	@$(CARGO) $(FMT)