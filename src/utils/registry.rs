use windows::{
    core::{Result, HSTRING, PCWSTR},
    Win32::System::Registry::{
        RegCloseKey, RegCreateKeyExW, RegDeleteTreeW, RegSetValueExW, HKEY, KEY_WRITE, REG_OPTION_NON_VOLATILE, REG_SZ
    },
};

use crate::check_win32;
use crate::utils::winutils::to_wide;

pub trait RegKey {
    fn create_subkey(&self, subkey: &str) -> Result<HKEY>;
    fn set_string(&self, value_name: &str, value: &str) -> Result<()>;
    fn delete_tree(&self, subkey: &str) -> Result<()>;
    fn close(&self) -> Result<()>;
}

impl RegKey for HKEY {
    fn create_subkey(&self, subkey_name: &str) -> Result<HKEY> {
        let subkey_name_w = HSTRING::from(subkey_name);
        let mut subkey_handle: HKEY = HKEY::default();

        unsafe {
            let result = RegCreateKeyExW(
                *self,
                PCWSTR(subkey_name_w.as_ptr()),
                0,
                None,
                REG_OPTION_NON_VOLATILE,
                KEY_WRITE,
                None,
                &mut subkey_handle,
                None,
            );

            check_win32!(result, subkey_handle)
        }
    }

    fn set_string(&self, value_name: &str, value: &str) -> Result<()> {
        let value_name_w = HSTRING::from(value_name);
        let value_w = to_wide(value);
        unsafe {
            let result = RegSetValueExW(
                *self,
                PCWSTR(value_name_w.as_ptr()),
                0,
                REG_SZ,
                Some(value_w.as_slice())
            );

            check_win32!(result)
        }
    }

    fn delete_tree(&self, subkey: &str) -> Result<()> {
        let subkey_w = HSTRING::from(subkey);
        unsafe {
            let result = RegDeleteTreeW(*self, PCWSTR(subkey_w.as_ptr()));

            check_win32!(result)
        }
    }

    fn close(&self) -> Result<()> {
        unsafe {
            let result = RegCloseKey(*self);
            check_win32!(result)
        }
    }
}