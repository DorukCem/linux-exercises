/*
    The tee command reads its standard input until end-of-file, writing a copy of the input
    to standard output and to the file named in its command-line argument.
    Implement tee using I/O system calls.
*/

use std::os::fd::BorrowedFd;

use nix::{
    fcntl::{open, OFlag},
    libc::{BUFSIZ, STDOUT_FILENO},
    sys::stat::Mode,
    unistd::{close, read, write},
};

fn main() {
    let mut args: Vec<_> = std::env::args().into_iter().collect();
    if args.len() != 3 {
        println!("Expected 3 arguments")
    }

    let output = args.remove(2);
    let input = args.remove(1);

    let input_fd = open(
        input.as_str(),
        OFlag::O_RDONLY,
        nix::sys::stat::Mode::S_IRUSR,
    )
    .unwrap();

    let output_fd = open(
        output.as_str(),
        OFlag::O_CREAT | OFlag::O_WRONLY,
        Mode::from_bits_truncate(0o666),
    )
    .unwrap();

    let mut buf = [0; BUFSIZ as usize];
    loop {
        let bytes_read = read(input_fd, &mut buf).unwrap();
        if bytes_read == 0 {
            break;
        }
        write(
            unsafe { BorrowedFd::borrow_raw(output_fd) },
            &buf[..bytes_read],
        )
        .unwrap();

        // Write to stdout
        write(unsafe { BorrowedFd::borrow_raw(STDOUT_FILENO) }, &buf[..bytes_read]).unwrap();
    }


    close(input_fd).unwrap();
    close(output_fd).unwrap();
}
