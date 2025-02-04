/*
    After each of the calls to write() in the following code, explain what the content of
    the output file would be, and why:
    fd1 = open(file, O_RDWR | O_CREAT | O_TRUNC, S_IRUSR | S_IWUSR);
    fd2 = dup(fd1);
    fd3 = open(file, O_RDWR);
    write(fd1, "Hello,", 6);
    write(fd2, "world", 6);
    lseek(fd2, 0, SEEK_SET);
    write(fd1, "HELLO,", 6);
    write(fd3, "Gidday", 6);
*/

use std::os::fd::BorrowedFd;

use nix::{
    fcntl::{open, OFlag},
    sys::stat::Mode,
    unistd::{close, dup, lseek, write},
};

fn main() {
    let mut args: Vec<_> = std::env::args().into_iter().collect();
    if args.len() != 2 {
        panic!("Expected 1 argument")
    }

    let file = args.remove(1);

    let fd1 = open(
        file.as_str(),
        OFlag::O_RDWR | OFlag::O_CREAT | OFlag::O_TRUNC,
        Mode::S_IRUSR | Mode::from_bits_truncate(0o666),
    )
    .unwrap();
    let fd2 = dup(fd1).unwrap();
    let fd3 = open(
        file.as_str(),
        OFlag::O_RDWR,
        Mode::from_bits_truncate(0o666),
    )
    .unwrap();
    write(unsafe { BorrowedFd::borrow_raw(fd1) }, "Hello,".as_bytes()).unwrap();
    write(unsafe { BorrowedFd::borrow_raw(fd2) }, "world".as_bytes()).unwrap(); // At this point we have written "Hello, world"
    lseek(fd2, 0, nix::unistd::Whence::SeekSet).unwrap();
    write(unsafe { BorrowedFd::borrow_raw(fd1) }, "HELLO,".as_bytes()).unwrap(); // At this point we have written "HELLP, world"
    write(unsafe { BorrowedFd::borrow_raw(fd3) }, "Gidday".as_bytes()).unwrap(); // At this point we have written "Gidday, world"

    // ? Because fd3 is not a dup of fd1 its offsett moves independatly from fd1

    close(fd1).unwrap();
    close(fd2).unwrap();
    close(fd3).unwrap();
}
