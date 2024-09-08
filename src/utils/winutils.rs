use std::borrow::BorrowMut;

use windows::{
    core::{Interface, Result, GUID},
    Win32::{
        Foundation::{HANDLE, HMODULE, MAX_PATH},
        Storage::FileSystem::WriteFile,
        System::{
            Com::{CoCreateInstance, CLSCTX_INPROC_SERVER},
            LibraryLoader::GetModuleFileNameW,
        },
    },
};

use crate::dll::DllModule;

#[derive(serde::Serialize)]
struct KeyEvent {
    r#type: String,
    message: String,
}

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
    Ok(unsafe { CoCreateInstance(clsid, None, CLSCTX_INPROC_SERVER)? })
}

pub fn to_wide(s: &str) -> Vec<u8> {
    let mut wide: Vec<u8> = s
        .encode_utf16()
        .flat_map(|c| c.to_le_bytes())
        .collect::<Vec<u8>>();
    wide.push(0);
    return wide;
}

pub fn to_wide_16(s: &str) -> Vec<u16> {
    let mut wide: Vec<u16> = s.encode_utf16().collect();
    wide.push(0);
    return wide;
}

pub fn get_module_path() -> String {
    unsafe {
        // Get a handle to the current module
        let dll_instance = DllModule::global().lock().unwrap();
        let h_module: HMODULE = dll_instance.hinst;

        // Get the module file name
        let mut buffer: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];
        let length = GetModuleFileNameW(h_module, &mut buffer);

        // Convert the wide string to a Rust string
        String::from_utf16_lossy(&buffer[..length as usize])
    }
}

pub fn debug(handle: HANDLE, message: &str) -> Result<()> {
    let message = serde_json::to_string(&KeyEvent {
        r#type: "debug".to_string(),
        message: message.to_string(),
    })
    .unwrap();

    let wide: Vec<u8> = message.to_string().into_bytes();
    let message_len = wide.len();
    unsafe {
        WriteFile(
            handle,
            Some(&wide),
            Some((message_len as u32).borrow_mut()),
            None,
        )
    }?;
    Ok(())
}
