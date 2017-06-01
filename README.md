# picker

Pre-shim EFI bootloader for Container Linux.

## Building

### Prerequisites
* Nightly Rust (`rustup toolchain install nightly && rustup default nightly` if
  you're using rustup)
* Xargo (`cargo install xargo`)
* GNU EFI (install through your package manager)

### Building
For a release build:
```
make
```

For a debug build:
```
make debug
```

The resulting EFI application will reside in `build/`.
