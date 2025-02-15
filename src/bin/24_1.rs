/* When a fork() is performed, the child receives duplicates of all of the parent’s file
descriptors. */

use std::{fs::File, os::fd::AsFd};

use nix::{
    libc::sleep,
    unistd::{fork, write, ForkResult},
};

fn main() {
    let fd = File::create("test/forked").unwrap();

    match unsafe { fork() } {
        Ok(ForkResult::Parent { .. }) => {
            unsafe { sleep(1) }; // Give time for the child to execute
        }
        Ok(ForkResult::Child) => {}

        Err(_) => println!("Fork failed"),
    }
    let pid = std::process::id();
    write(
        fd.as_fd(),
        format!("Writing from program with pid: {pid}\r\n").as_bytes(),
    )
    .unwrap();
}
