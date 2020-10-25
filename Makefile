.PHONY: all amd64 crate_esp run clean install-depends build rcore-fs-fuse pack

all: amd64 run

BOOTLOADER_DIR := bootloader
OVMF = ${BOOTLOADER_DIR}/OVMF.fd
USER_DIR = user
OUT_IMG = build/SYS.img
USER_QCOW2 = build/x86_64.qcow2
rcore_fs_fuse_revision := 7f5eeac

# build
build:
	@cd bootloader && cargo build --release
	@cd kernel && cargo build  --release
	@make pack

# 编译bootloader和kernel
amd64:
	@make build
	@make crate_esp

# x86_64编译出的文件目录
crate_esp:
	@mkdir -p build/pc/esp/EFI/kernel build/pc/esp/EFI/Boot
	@cp target/x86_64-unknown-uefi/release/bootloader.efi build/pc/esp/EFI/Boot/BootX64.efi
	@cp target/amd64/release/kernel build/pc/esp/EFI/kernel/kernel.elf

# QEMU运行x86_64
run:
	@qemu-system-x86_64 \
    -bios ${OVMF} \
    -drive format=raw,file=fat:rw:build/pc/esp \
	-drive format=qcow2,file=$(USER_QCOW2),media=disk,cache=writeback,id=sfsimg,if=none \
	-device ahci,id=ahci0 \
	-device ide-hd,drive=sfsimg,bus=ahci0.0
    -m 4096 \
    -smp 2 \
    -serial mon:stdio \
   # -nographic \

# 清理编译出来的文件
clean:
	@cargo clean
	@rm -rf build

# 依赖安装
install-depends:
	rustup install nightly
	rustup default nightly
	rustup component add rust-src
	rustup component add llvm-tools-preview

# 安装依赖
rcore-fs-fuse:
ifneq ($(shell rcore-fs-fuse dir image git-version), $(rcore_fs_fuse_revision))
	@echo Installing rcore-fs-fuse
	@cargo install rcore-fs-fuse --git https://github.com/rcore-os/rcore-fs --rev $(rcore_fs_fuse_revision) --force
endif

# 打包文件
pack:
	@mkdir -p build
	@rcore-fs-fuse $(OUT_IMG) $(USER_DIR) zip
	@qemu-img convert -f raw $(OUT_IMG) -O qcow2 $(USER_QCOW2)
	@qemu-img resize $(USER_QCOW2) +1G