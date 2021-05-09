use std::ffi::{
    CStr,
    CString,
};

pub fn vk_to_string(i8_buf: &[i8]) -> String {
    let mut s = String::new();
    for &i in i8_buf {
        if i == 0 {
            break;
        }

        let c: char = (i as u8) as char;
        s.push(c);
    }

    s
}

// Safety: i8_buf is a valid C string
pub unsafe fn vk_to_cstring(i8_buf: &[i8]) -> CString {
    CStr::from_ptr(i8_buf.as_ptr()).to_owned()
}

pub fn string_vec_to_ptr_vec(storage: &[CString]) -> Vec<*const i8> {
    storage.into_iter().map(|s| s.as_ptr()).collect()
}
