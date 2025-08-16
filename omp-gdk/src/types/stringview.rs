use encoding::all::WINDOWS_874;
use encoding::{EncoderTrap, Encoding};
use std::ffi::CStr;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct StringView {
    data: *const std::ffi::c_char,

    size: usize,
}

impl Default for StringView {
    fn default() -> Self {
        Self::new()
    }
}

impl StringView {
    pub fn new() -> Self {
        StringView {
            data: std::ptr::null(),
            size: 0,
        }
    }

    pub fn get_data(&self) -> String {
        unsafe {
            if self.data.is_null() {
                return String::from("<null>");
            }
            println!("data: {:?}", CStr::from_ptr(self.data).to_string_lossy());
            match CStr::from_ptr(self.data).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => WINDOWS_874
                    .decode(
                        CStr::from_ptr(self.data).to_bytes(),
                        encoding::DecoderTrap::Replace,
                    )
                    .unwrap_or_else(|_| String::from("<decode error>")),
            }
        }
    }
}

impl From<&str> for StringView {
    fn from(value: &str) -> Self {
        println!("StringView::from() called with value: {:?}", value);

        let ansi_encoded = WINDOWS_874
            .encode(value, EncoderTrap::Strict)
            .map_err(|e| format!("encode failed: {}", e))
            .unwrap_or(b"????????????????????".to_vec());

        let c_string = std::ffi::CString::new(ansi_encoded)
            .map_err(|e| format!("CString failed: {}", e))
            .unwrap_or_else(|_| std::ffi::CString::new(b"????????????????????".to_vec()).unwrap());
        let data = c_string.as_ptr();
        let size = c_string.to_bytes().len();

        std::mem::forget(c_string);
        StringView { data, size }
    }
}
