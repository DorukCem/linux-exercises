/* Create any anonymous mapping which means that there will be no file and the memory obtained can be used like a buffer */

use std::num::NonZero;

use nix::sys::mman::{mmap_anonymous, munmap, MapFlags, ProtFlags};

fn main() {
    const BUFFSIZE: usize = 8;
    let ptr = unsafe {
        // This is Nix wrapper, we would normally pass -1 as fd to mmap and set flag to ANONYMOUS
        mmap_anonymous( 
            None,
            NonZero::new(BUFFSIZE).unwrap(),
            ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
            MapFlags::MAP_PRIVATE ,
        )
    }
    .expect("Failed to mmap");

    let buffer = unsafe { std::slice::from_raw_parts(ptr.as_ptr() as *const u8, BUFFSIZE) };
    println!("{:?}", buffer);

    // Unmap memory
    unsafe { munmap(ptr, BUFFSIZE).expect("Failed to unmap") };
}
