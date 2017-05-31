extern crate uefi;

pub mod util;

#[no_mangle]
pub extern "win64" fn efi_entry(image_handle: uefi::Handle,
                                system_table: *const uefi::SystemTable)
                                -> isize {
    uefi::set_system_table(system_table).console().reset();
    ::efi_main() as isize
}
