/* Using a fifo */

use nix::{
    fcntl::{open, OFlag},
    libc::{close, BUFSIZ},
    sys::stat::Mode,
    unistd::{read, write},
};

const PATH: &str = "test/fifo1";

fn main() {
    let fd = open(PATH, OFlag::O_RDONLY, Mode::from_bits_truncate(0o666)).unwrap();
    if fd < 0 {
        panic!("Open error");
    }
    let mut buf = [0; BUFSIZ as usize];
    let bytes_read = read(fd, &mut buf).unwrap();
    write(std::io::stdout(), &buf[..bytes_read]).unwrap();

    unsafe { close(fd) };
}
