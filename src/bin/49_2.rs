/* Read a file by mapping it to memory */

use std::{fs::OpenOptions, num::NonZero};

use nix::
    sys::mman::{mmap, msync, munmap, MapFlags, MsFlags, ProtFlags}
;

fn main() {
    let path = "test/mapped.txt";
    // Open file with read & write permissions
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .unwrap();
    let len: usize = 4096; // 4KB file size
    file.set_len(len as u64).unwrap();

    let ptr = unsafe {
        mmap(
            None,
            NonZero::new(len as usize).unwrap(),
            ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
            MapFlags::MAP_SHARED, // Allow writing changes back to file
            &file,
            0,
        )
    }
    .expect("Failed to mmap");

    let data = unsafe { std::slice::from_raw_parts_mut(ptr.as_ptr() as *mut u8, len) };
    let s = b"Hello from shared memory!!";
    data[..s.len()].copy_from_slice(s);

    file.set_len(s.len() as u64).unwrap(); // The editor cannot open the file if it has been zero padded so we truncate it back
    // Ensure changes are written to disk
    unsafe { msync(ptr, len, MsFlags::MS_SYNC).expect("Failed to sync memory") };
    // Unmap memory
    unsafe { munmap(ptr, len).expect("Failed to unmap") };
}
