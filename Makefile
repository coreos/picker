LIB_DIR ?= /usr/lib64
EFI_DIR ?= $(LIB_DIR)/gnuefi

all: TYPE := release
all: XARGO_FLAG := --release
all: build/bootx64-release.efi
.PHONY: all

debug: TYPE := debug
debug: XARGO_FLAG :=
debug: build/bootx64-debug.efi
.PHONY: debug

build/bootx64-%.efi: build/picker-%.so
	objcopy -j .text -j .sdata -j .data -j .dynamic -j .dynsym -j .rel -j .rela -j .reloc --target=efi-app-x86_64 $< build/bootx64-$(TYPE).efi

build/picker-%.so: target/efi-app-x86_64/%/deps/picker.o
	@mkdir -p build
	ld target/efi-app-x86_64/$(TYPE)/deps/*.o $(EFI_DIR)/crt0-efi-x86_64.o -nostdlib -znocombreloc -T $(EFI_DIR)/elf_x86_64_efi.lds -shared -Bsymbolic -lefi -L $(LIB_DIR) -pie -e efi_entry -o build/picker-$(TYPE).so

target/efi-app-x86_64/%/deps/picker.o: src/picker.rs src/uefi_entry/mod.rs src/uefi_entry/util.rs Cargo.toml
	xargo build --target=efi-app-x86_64 $(XARGO_FLAG)

clean:
	-rm build/*
	-xargo clean
.PHONY: clean

.PRECIOUS: target/efi-app-x86_64/%/deps/picker.o
