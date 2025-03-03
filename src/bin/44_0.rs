use std::os::fd::AsRawFd;

use nix::{
    libc::BUFSIZ,
    sys::wait::waitpid,
    unistd::{fork, pipe, read, write, ForkResult},
};

fn main() {
    let (reader, writer) = pipe().unwrap();
    /*To connect two processes using a pipe, we follow the pipe() call with a call to fork(). Dur-
    ing a fork(), the child process inherits copies of its parent’s file descriptors */
    match unsafe { fork().unwrap() } {
        /*While it is possible for the parent and child to both read from and write to the
        pipe, this is not usual. After the fork(), one process closes its
        descriptor for the write end of the pipe, and the other closes its descriptor for the
        read end. */
        ForkResult::Parent { child } => {
            drop(reader); // Since nix provides this abstraction we must drop it instead of closing it.
            write(writer, "Hello from parent!\n".as_bytes()).unwrap();
            let _ = waitpid(child, None);
        }
        ForkResult::Child => {
            drop(writer);
            let mut buf = [0; BUFSIZ as usize];
            let bytes = read(reader.as_raw_fd(), &mut buf).unwrap();
            write(std::io::stdout(), &buf[..bytes]).unwrap();
        }
    }
}
