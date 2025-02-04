/*
    Write a program to verify that duplicated file descriptors share a file offset value
    and open file status flags.
*/
use nix::{fcntl::{open, OFlag}, sys::stat::Mode, unistd::{close, dup, lseek}};

fn main() {
    let mut args: Vec<_> = std::env::args().into_iter().collect();
    if args.len() != 2 {
        panic!("Expected 1 argument")
    }

    let input = args.remove(1);

    let fd = open(
        input.as_str(),
        OFlag::O_RDONLY,
        Mode::from_bits_truncate(0o666),
    )
    .unwrap();

    let fd2 = dup(fd).unwrap(); // ? Duplicating a fd ensures we get the same offset, etc..

    let offset_1 = lseek(fd, 0, nix::unistd::Whence::SeekCur).unwrap();
    let offset_2 = lseek(fd2, 0, nix::unistd::Whence::SeekCur).unwrap();
    close(fd).unwrap();
    close(fd2).unwrap(); // ? If you want to ensure proper cleanup, close both descriptors.
    
    assert_eq!(offset_1, offset_2);
    println!("Success !!")
}
