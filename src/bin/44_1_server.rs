use std::os::fd::BorrowedFd;

use nix::{fcntl::{open, OFlag}, libc::close, sys::stat::{self, Mode}, unistd::{mkfifo,write}};


const PATH: &str = "test/fifo1";

fn main(){
    mkfifo(PATH, stat::Mode::S_IRWXU).unwrap();
    let fd = open(PATH, OFlag::O_WRONLY, Mode::from_bits_truncate(0o666)).unwrap();
    if fd <0 {
        panic!("Open error");
    }
    write(unsafe { BorrowedFd::borrow_raw(fd) }, "Hello from writer!\n".as_bytes()).unwrap();
    unsafe { close(fd) };
}