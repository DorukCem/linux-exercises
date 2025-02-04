// 5-2.Write a program that opens an existing file for writing with the O_APPEND flag, and
// then seeks to the beginning of the file before writing some data. Where does the
// data appear in the file? Why?

use std::os::fd::BorrowedFd;

use nix::{
    fcntl::{open, OFlag},
    sys::stat::Mode,
    unistd::{self, lseek ,write},
};

fn main() {
    let mut args: Vec<_> = std::env::args().into_iter().collect();
    if args.len() != 2 {
        println!("Expected 1 argument")
    }

    let input = args.remove(1);

    let fd = open(
        input.as_str(),
        OFlag::O_CREAT | OFlag::O_WRONLY | OFlag::O_APPEND,
        Mode::from_bits_truncate(0o666),
    )
    .unwrap();

    let _offset = lseek( fd , 0, unistd::Whence::SeekSet).unwrap();
    let buf = "xxxxxx";
    write(unsafe { BorrowedFd::borrow_raw(fd) }, buf.as_bytes()).unwrap();
}

// Answer: when a file is opened with the O_APPEND flag, all writes will be appended to the end of the file regardless of the file offset set by lseek()