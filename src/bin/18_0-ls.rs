use nix::libc::{opendir, readdir};
use std::ffi::{CStr, CString};

// List all files in the working directory

fn main() {
    let path = CString::new(".").unwrap();
    let dirp = unsafe { opendir(path.as_ptr()) };
    if dirp.is_null() {
        panic!("opendir failed on: {path:?}")
    }

    loop {
        let dp = unsafe { readdir(dirp) };
        if dp.is_null() {
            break;
        }
        let name = unsafe { CStr::from_ptr(((*dp).d_name).as_ptr()) };
        let name = name.to_str().unwrap();
        if name == "." || name == ".." {
            continue;
        }
        println!("{name}");
    }
}
