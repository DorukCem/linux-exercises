/* Read a file by mapping it to memory */

use std::num::NonZero;

use nix::sys::mman::{mmap, munmap, MapFlags, ProtFlags};

fn main() {
    let file = std::fs::File::open("test/anna.txt").unwrap();
    let len = file.metadata().unwrap().len() as usize;

    let ptr = unsafe {
        mmap(
            None,
            NonZero::new(len).unwrap(),
            ProtFlags::PROT_READ,
            MapFlags::MAP_PRIVATE,
            file,
            0,
        )
    }
    .expect("Failed to mmap");

    let data = unsafe { std::slice::from_raw_parts(ptr.as_ptr() as *const u8, len) };
    println!("File contents: {:?}", std::str::from_utf8(data));

    // Unmap memory
    unsafe { munmap(ptr, len).expect("Failed to unmap") };
}
