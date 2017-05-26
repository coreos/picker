// These definitions are all derived from the UEFI specification.
//
// For the most part, function pointer types are defined as *const () until they are actually
// needed by something in the rest of picker. By the same token, types that are only needed in the
// definition of other types are typically not defined as public.

pub type EFI_HANDLE = *const ();
pub type EFI_STATUS = usize;
pub type EFI_EVENT = *const ();
pub type EFI_TPL = usize;
pub type EFI_EVENT_NOTIFY = *const ();

type EFI_TEXT_RESET = extern "win64" fn(*const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, u8);
type EFI_TEXT_STRING = extern "win64" fn(*const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, *const u16);
type EFI_INPUT_READ_KEY = extern "win64" fn(*const EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
                                            *const EFI_INPUT_KEY);

type EFI_RAISE_TPL = *const ();
type EFI_RESTORE_TPL = *const ();

type EFI_ALLOCATE_PAGES = *const ();
type EFI_FREE_PAGES = *const ();
type EFI_GET_MEMORY_MAP = *const ();
type EFI_ALLOCATE_POOL = extern "win64" fn(EFI_MEMORY_TYPE, usize, *mut *mut u8);
type EFI_FREE_POOL = *const ();

type EFI_CREATE_EVENT = extern "win64" fn(u32,
                                          EFI_TPL,
                                          EFI_EVENT_NOTIFY,
                                          *const (),
                                          *mut EFI_EVENT);
pub type EFI_SET_TIMER = extern "win64" fn(EFI_EVENT, EFI_TIMER_DELAY, u64);
type EFI_WAIT_FOR_EVENT = extern "win64" fn(usize, *const EFI_EVENT, *mut usize);
type EFI_SIGNAL_EVENT = extern "win64" fn(EFI_EVENT);
type EFI_CLOSE_EVENT = extern "win64" fn(EFI_EVENT);
type EFI_CHECK_EVENT = extern "win64" fn(EFI_EVENT) -> EFI_STATUS;

type EFI_INSTALL_PROTOCOL_INTERFACE = *const ();
type EFI_REINSTALL_PROTOCOL_INTERFACE = *const ();
type EFI_UNINSTALL_PROTOCOL_INTERFACE = *const ();
type EFI_HANDLE_PROTOCOL = extern "win64" fn(EFI_HANDLE, *const EFI_GUID, *mut *const ())
                                             -> EFI_STATUS;
type EFI_REGISTER_PROTOCOL_NOTIFY = *const ();
type EFI_LOCATE_HANDLE = *const ();
type EFI_LOCATE_DEVICE_PATH = *const ();
type EFI_INSTALL_CONFIGURATION_TABLE = *const ();

type EFI_IMAGE_LOAD = extern "win64" fn(u8,
                                        EFI_HANDLE,
                                        *const EFI_DEVICE_PATH_PROTOCOL,
                                        *const (),
                                        usize,
                                        *mut EFI_HANDLE)
                                        -> EFI_STATUS;
type EFI_IMAGE_START = extern "win64" fn(EFI_HANDLE, *mut usize, *mut ()) -> EFI_STATUS;
type EFI_EXIT = extern "win64" fn(EFI_HANDLE, EFI_STATUS, usize, *const u16);
type EFI_IMAGE_UNLOAD = *const ();
type EFI_EXIT_BOOT_SERVICES = extern "win64" fn(EFI_HANDLE, usize);

type EFI_GET_NEXT_MONOTONIC_COUNT = *const ();
type EFI_STALL = *const ();
type EFI_SET_WATCHDOG_TIMER = *const ();

type EFI_CONNECT_CONTROLLER = *const ();
type EFI_DISCONNECT_CONTROLLER = *const ();

type EFI_OPEN_PROTOCOL = extern "win64" fn(EFI_HANDLE,
                                           *const EFI_GUID,
                                           *mut *const (),
                                           EFI_HANDLE,
                                           EFI_HANDLE,
                                           u32);
type EFI_CLOSE_PROTOCOL = *const ();
type EFI_OPEN_PROTOCOL_INFORMATION = *const ();

type EFI_PROTOCOLS_PER_HANDLE = *const ();
type EFI_LOCATE_HANDLE_BUFFER = *const ();
type EFI_LOCATE_PROTOCOL = *const ();
type EFI_INSTALL_MULTIPLE_PROTOCOL_INTERFACES = *const ();
type EFI_UNINSTALL_MULTIPLE_PROTOCOL_INTERFACES = *const ();

type EFI_CALCULATE_CRC32 = *const ();

type EFI_COPY_MEM = *const ();
type EFI_SET_MEM = extern "win64" fn(*mut u8, usize, u8);
type EFI_CREATE_EVENT_EX = *const ();

#[repr(C)]
struct EFI_TABLE_HEADER {
    Signature: u64,
    Revision: u32,
    HeaderSize: u32,
    CRC32: u32,
    Reserved: u32,
}

#[repr(C)]
pub struct EFI_SYSTEM_TABLE {
    Hdr: EFI_TABLE_HEADER,
    FirmwareVendor: *const u16,
    FirmwareRevision: u32,
    ConsoleInHandle: EFI_HANDLE,
    pub ConIn: *const EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    ConsoleOutHandle: EFI_HANDLE,
    pub ConOut: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    StandardErrorHandle: EFI_HANDLE,
    StdErr: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    RuntimeServices: *const EFI_RUNTIME_SERVICES,
    pub BootServices: *const EFI_BOOT_SERVICES,
    NumberOfTableEntries: usize,
    ConfigurationTable: *const EFI_CONFIGURATION_TABLE,
}

#[repr(C)]
pub struct EFI_SIMPLE_TEXT_INPUT_PROTOCOL {
    pub Reset: EFI_TEXT_RESET,
    pub ReadKeyStroke: EFI_INPUT_READ_KEY,
    pub WaitForKey: EFI_EVENT,
}

#[repr(C)]
pub struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    pub Reset: EFI_TEXT_RESET,
    pub OutputString: EFI_TEXT_STRING,
}

#[repr(C)]
pub struct EFI_BOOT_SERVICES {
    Hdr: EFI_TABLE_HEADER,

    // Task priority services
    RaiseTPL: EFI_RAISE_TPL,
    RestoreTPL: EFI_RESTORE_TPL,

    // Memory services
    AllocatePages: EFI_ALLOCATE_PAGES,
    FreePages: EFI_FREE_PAGES,
    GetMemoryMap: EFI_GET_MEMORY_MAP,
    pub AllocatePool: EFI_ALLOCATE_POOL,
    FreePool: EFI_FREE_POOL,

    // Event & timer services
    pub CreateEvent: EFI_CREATE_EVENT,
    pub SetTimer: EFI_SET_TIMER,
    pub WaitForEvent: EFI_WAIT_FOR_EVENT,
    SignalEvent: EFI_SIGNAL_EVENT,
    CloseEvent: EFI_CLOSE_EVENT,
    CheckEvent: EFI_CHECK_EVENT,

    // Protocol handler services
    InstallProtocolInterface: EFI_INSTALL_PROTOCOL_INTERFACE,
    ReinstallProtocolInterface: EFI_REINSTALL_PROTOCOL_INTERFACE,
    UninstallProtocolInterface: EFI_UNINSTALL_PROTOCOL_INTERFACE,
    pub HandleProtocol: EFI_HANDLE_PROTOCOL,
    Reserved: *const (),
    RegisterProtocolNotify: EFI_REGISTER_PROTOCOL_NOTIFY,
    LocateHandle: EFI_LOCATE_HANDLE,
    LocateDevicePath: EFI_LOCATE_DEVICE_PATH,
    InstallConfigurationTable: EFI_INSTALL_CONFIGURATION_TABLE,

    // Image services
    pub LoadImage: EFI_IMAGE_LOAD,
    pub StartImage: EFI_IMAGE_START,
    Exit: EFI_EXIT,
    UnloadImage: EFI_IMAGE_UNLOAD,
    ExitBootServices: EFI_EXIT_BOOT_SERVICES,

    // Misc. services
    GetNextMonotonicCount: EFI_GET_NEXT_MONOTONIC_COUNT,
    Stall: EFI_STALL,
    SetWatchdogTimer: EFI_SET_WATCHDOG_TIMER,

    // Driver support services
    ConnectController: EFI_CONNECT_CONTROLLER,
    DisconnectController: EFI_DISCONNECT_CONTROLLER,

    // Open/close protocol services
    OpenProtocol: EFI_OPEN_PROTOCOL,
    CloseProtocol: EFI_CLOSE_PROTOCOL,
    OpenProtocolInformation: EFI_OPEN_PROTOCOL_INFORMATION,

    // Library services
    ProtocolsPerHandle: EFI_PROTOCOLS_PER_HANDLE,
    LocateHandleBuffer: EFI_LOCATE_HANDLE_BUFFER,
    LocateProtocol: EFI_LOCATE_PROTOCOL,
    InstallMultipleProtocolInterfaces: EFI_INSTALL_MULTIPLE_PROTOCOL_INTERFACES,
    UninstallMultipleProtocolInterfaces: EFI_UNINSTALL_MULTIPLE_PROTOCOL_INTERFACES,

    // CRC32 services
    CalculateCrc32: EFI_CALCULATE_CRC32,

    // Misc. services
    CopyMem: EFI_COPY_MEM,
    pub SetMem: EFI_SET_MEM,
    CreateEventEx: EFI_CREATE_EVENT_EX,
}

#[repr(C)]
struct EFI_RUNTIME_SERVICES {
    Hdr: EFI_TABLE_HEADER, // Fill in the rest of this later, as needed, I guess.
}

#[repr(C)]
struct EFI_CONFIGURATION_TABLE {
    VendorGuid: EFI_GUID,
    VendorTable: *const (),
}

#[repr(C)]
pub struct EFI_GUID {
    Data1: u32,
    Data2: u16,
    Data3: u16,
    Data4: [u8; 8],
}

#[repr(C)]
pub struct EFI_INPUT_KEY {
    pub ScanCode: u16,
    pub UnicodeChar: u16,
}

#[repr(C)]
pub struct EFI_DEVICE_PATH_PROTOCOL {
    pub Type: u8,
    pub SubType: u8,
    pub Length: [u8; 2],
}

#[repr(C)]
pub struct EFI_LOADED_IMAGE_PROTOCOL {
    Revision: u32,
    ParentHandle: EFI_HANDLE,
    SystemTable: *const EFI_SYSTEM_TABLE,

    DeviceHandle: EFI_HANDLE,
    pub FilePath: *const EFI_DEVICE_PATH_PROTOCOL,
    Reserved: *const (),

    LoadOptionsSize: u32,
    LoadOptions: *const (),

    ImageBase: *const (),
    ImageSize: u64,
    ImageCodeType: EFI_MEMORY_TYPE,
    ImageDataType: EFI_MEMORY_TYPE,
    Unload: EFI_IMAGE_UNLOAD,
}

#[repr(C)]
pub enum EFI_TIMER_DELAY {
    TimerCancel,
    TimerPeriodic,
    TimerRelative,
}

#[repr(C)]
pub enum EFI_MEMORY_TYPE {
    EfiReservedMemoryType,
    EfiLoaderCode,
    EfiLoaderData,
    EfiBootServicesCode,
    EfiBootServicesData,
    EfiRuntimeServicesCode,
    EfiRuntimeServicesData,
    EfiConventionalMemory,
    EfiUnusableMemory,
    EfiACPIReclaimMemory,
    EfiACPIMemoryNVS,
    EfiMemoryMappedIO,
    EfiMemoryMappedIOPortSpace,
    EfiPalCode,
    EfiPersistentMemory,
    EfiMaxMemoryType,
}

pub const EVT_TIMER: u32 = 0x80000000;
pub const EVT_RUNTIME: u32 = 0x40000000;
pub const EVT_NOTIFY_WAIT: u32 = 0x00000100;
pub const EVT_NOTIFY_SIGNAL: u32 = 0x00000200;

pub const TPL_APPLICATION: EFI_TPL = 4;
pub const TPL_CALLBACK: EFI_TPL = 8;
pub const TPL_NOTIFY: EFI_TPL = 16;
pub const TPL_HIGH_LEVEL: EFI_TPL = 31;

pub const EFI_LOADED_IMAGE_PROTOCOL_GUID: EFI_GUID = EFI_GUID {
    Data1: 0x5b1b31a1,
    Data2: 0x9562,
    Data3: 0x11d2,
    Data4: [0x8e, 0x3f, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
};
