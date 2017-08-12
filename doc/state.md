# picker's current state

## Build system
Currently, picker is built as a Rust library. When built, `cargo build` outputs object files for
picker and all of its dependencies into `target/(release|debug)/deps`. `make` invokes `cargo build`,
links those object files into a shared library `picker.so`, and uses `objcopy` to make an EFI
executable from the shared library.

## UEFI entry point
`picker::uefi_entry` contains the actual entry point for picker. All it does is initialize [the
rust-uefi library](https://github.com/csssuf/rust-uefi) and call `picker::efi_main`.

## rust-uefi
The rust-uefi library is responsible for actually wrapping UEFI structs and providing a more
idiomatic Rust interface to the protocols UEFI provides. Functionality has been added to it largely
as needed by picker, so it's by no means a complete implementation of the UEFI specification. The
Rust interface it provides could also use some refinement; it still deals with a lot of raw pointers
that could be references, and it has a number of functions which return memory that must be
explicitly deallocated by the caller. This obviously isn't ideal, but until the new `Alloc` trait
and other allocator API redesigns in Rust are complete and stable, improving it mostly takes the
form of manually adding `Drop` implementations.

### Allocation
The biggest pain point of the rust-uefi library is currently allocation. In general, any memory
rust-uefi allocates via UEFI's `AllocatePool` or `AllocatePages` functions must be manually freed by
the library consumer. The current plan to improve this situation is to implement wrapper structs in
rust-uefi for any data structures that require allocation. This will allow rust-uefi to implement
`Drop` for all of these structs, in turn removing the need for library consumers to manually free
them.

## picker's behavior
On boot, picker attempts to read the GPT header from the disk picker was booted from. If the disk
contains a valid GPT header, it then determines what, if any, the next gptprio partition to try is.
The user is then prompted (over UEFI console and serial if possible) to choose a partition to boot.
If the user does not choose a partition within 5 seconds, the gptprio result is used (or USR-A if no
eligible gptprio partition is found). *Note that picker's gptprio implementation does not update
`tries`.* The partition handling code has been designed with this in mind, however; when writing GPT
partition entries, the unused portion should be set to zero. picker's GPT partition handling reads
in the entire partition entry array and stores it in the `raw_partitions` field of `GptDisk`, and
stores references into that data as `partitions`. This way, when the partitions are updated,
`raw_partitions` remains as a contiguous partition entry array that can simply be written to disk,
without needing to reassemble the individual entries into a valid array.

Once a partition has been chosen, the shim for that partition is booted. Shim expects the
`LoadOptions` field in its `LoadedImageProtocol` to be set to the following (as a UTF-16 string):
```
\path\to\shim.efi \nextstage.efi
```
*Note that `\nextstage.efi` is given relative to the path to shim.* Thus, if shim is stored in
`\dir1\dir2\shim.efi` and needs to load `\dir1\dir2\next.efi`, the second path should just be
`\next.efi`.

Picker uses `\EFI\coreos\shim_a.efi` and `\EFI\coreos\shim_b.efi` as its USR-A and USR-B shims,
respectively, and instructs shim to find grub at `\EFI\coreos\grub_a.efi` and
`\EFI\coreos\grub_b.efi`, respectively.

## Next steps
Picker doesn't tell grub which partition to boot. Fortunately, the Container Linux grub has patches
allowing it to read EFI variables, which are a much more convenient way to pass information than
`LoadOptions`. Implementing this would likely take the form of having picker set an EFI variable
like `PickerChoice`, and editing `grub.cfg` to check for and use that variable before it runs the
grub gptprio implementation.
