LIB_DIR ?= /usr/lib64
EFI_DIR ?= $(LIB_DIR)/gnuefi

all: TYPE := release
all: XARGO_FLAG := --release
all: build/bootx64-release.efi
.PHONY: all

debug: TYPE := debug
debug: XARGO_FLAG :=
debug: build/bootx64-debug.efi build/bootx64-debug-symbols.efi
.PHONY: debug

build/bootx64-%.efi: build/picker-%.so
	objcopy -j .text -j .sdata -j .data -j .dynamic -j .dynsym -j .rel -j .rela -j .reloc --target=efi-app-x86_64 $< $@

build/bootx64-%-symbols.efi: build/picker-%.so
	objcopy --target=efi-app-x86_64 $< $@

# Because xargo/cargo don't remove old build artifacts from deps/ when
# building, if source files or dependencies are removed, they will still be
# linked into the final application unless they are cleaned up with the
# `clean` target or `xargo clean`.
build/picker-%.so: target/efi-app-x86_64/%/deps/picker.o
	@mkdir -p build
	ld target/efi-app-x86_64/$(TYPE)/deps/*.o $(EFI_DIR)/crt0-efi-x86_64.o -nostdlib -znocombreloc -T $(EFI_DIR)/elf_x86_64_efi.lds -shared -Bsymbolic -lefi -L $(LIB_DIR) -pie -e efi_entry -o $@

target/efi-app-x86_64/%/deps/picker.o: src/picker.rs src/uefi_entry/mod.rs src/util/mod.rs src/util/input.rs Cargo.toml
	xargo build --target=efi-app-x86_64 $(XARGO_FLAG)

clean:
	-rm build/*
	-xargo clean
.PHONY: clean

.PRECIOUS: target/efi-app-x86_64/%/deps/picker.o
