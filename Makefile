arch ?= x86_64
kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso
rust_os := target/$(arch)-unknown-linux-gnu/debug/libos.a
buildenv_name := os_buildenv
buildenv_source = buildenv

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard src/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm, \
	build/arch/$(arch)/%.o, $(assembly_source_files))

.PHONY: all clean run iso kernel test docker env

all: $(kernel)

clean:
	@rm -r build

run:
	@qemu-system-x86_64 -cdrom $(iso)

# Targets for generating a release (tests disabled) iso
iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@grub-mkrescue -o $(iso) build/isofiles 2> /dev/null
	@rm -r build/isofiles

$(kernel): kernel $(rust_os) $(assembly_object_files) $(linker_script)
	@ld -n -T $(linker_script) -o $(kernel) $(assembly_object_files) $(rust_os)

kernel:
	@RUST_TARGET_PATH=$(shell pwd) cargo build


# Targets for generating a tests enabled ISO
test: $(iso)_test

$(iso)_test: $(kernel)_test $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@grub-mkrescue -o $(iso) build/isofiles 2> /dev/null
	@rm -r build/isofiles

$(kernel)_test: kernel_test $(rust_os) $(assembly_object_files) $(linker_script)
	@ld -n -T $(linker_script) -o $(kernel) $(assembly_object_files) $(rust_os)

kernel_test:
	@RUST_TARGET_PATH=$(shell pwd) RUSTFLAGS="--cfg testing" cargo build

# compile assembly files
build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@

# Targets for generating a docker image for the build environment
# and for running it
docker:
	@docker run --rm -it -v $(shell pwd):/root/env $(buildenv_name)

env:
	@docker build $(buildenv_source) -t $(buildenv_name)

