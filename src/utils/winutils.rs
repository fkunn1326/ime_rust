use windows::{
    core::{
        w, Interface, Result, GUID, PCWSTR
    },
    Win32::{Foundation::{HMODULE, MAX_PATH}, System::{
        Com::{
            CoCreateInstance,
            CLSCTX_INPROC_SERVER
        }, LibraryLoader::{
            GetModuleFileNameW,
            GetModuleHandleExW,
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT
        }
    }, UI::WindowsAndMessaging::{MessageBoxW, MB_OK}},
};

pub trait GUIDExt {
    fn to_string(&self) -> String;
}

impl GUIDExt for GUID {
    fn to_string(&self) -> String {
        format!(
            "{{{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}}}",
            self.data1,
            self.data2,
            self.data3,
            self.data4[0],
            self.data4[1],
            self.data4[2],
            self.data4[3],
            self.data4[4],
            self.data4[5],
            self.data4[6],
            self.data4[7],
        )
    }
}


pub fn co_create_inproc<T: Interface>(clsid: &GUID) -> Result<T> {
    let iface: T = unsafe {
        CoCreateInstance(clsid, None, CLSCTX_INPROC_SERVER)?
    };
    Ok(iface)
}

pub fn to_wide(s: &str) -> Vec<u8> {
    let mut wide: Vec<u8> = s.encode_utf16().flat_map(|c| c.to_le_bytes()).collect::<Vec<u8>>();
    wide.push(0);
    wide
}

pub fn to_wide_16(s: &str) -> Vec<u16> {
    let mut wide: Vec<u16> = s.encode_utf16().collect();
    wide.push(0);
    return wide
}

pub fn get_module_path() -> String {
    unsafe {
        // Get a handle to the current module
        let mut h_module: HMODULE = HMODULE::default();
        let _ = GetModuleHandleExW(
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
            PCWSTR(get_module_path as *const _),
            &mut h_module,
        );

        // Get the module file name
        let mut buffer: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];
        let length = GetModuleFileNameW(h_module, &mut buffer);

        // Convert the wide string to a Rust string
        String::from_utf16_lossy(&buffer[..length as usize])
    }
}

pub fn alert(msg: &str) {
    unsafe {
        MessageBoxW(None, PCWSTR(to_wide_16(msg).as_ptr()), w!("Alert"), MB_OK);
    }
}