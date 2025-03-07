/* fcntl can be used to lock parts of a file */

use nix::{
    fcntl::{fcntl, FcntlArg::F_SETLK},
    libc::{flock, F_UNLCK, F_WRLCK, SEEK_SET},
};
use std::os::fd::AsRawFd;

fn main() {
    let file = std::fs::File::open("test/a.txt").unwrap();

    // lock bytes 0..5
    let _ = fcntl(
        file.as_raw_fd(),
        F_SETLK(&flock {
            l_type: F_WRLCK as i16,
            l_whence: SEEK_SET as i16,
            l_start: 0,
            l_len: 5,
            l_pid: 0,
        }),
    );

    let mut buffer = [0; 5 as usize];
    let b = nix::unistd::read(file.as_raw_fd(), &mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer[0..b]));

    // unlock bytes 0..5
    let _ = fcntl(
        file.as_raw_fd(),
        F_SETLK(&flock {
            l_type: F_UNLCK as i16,
            l_whence: SEEK_SET as i16,
            l_start: 0,
            l_len: 5,
            l_pid: 0,
        }),
    );

    
}
