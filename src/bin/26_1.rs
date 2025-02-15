/*
    If a child’s parent terminates, the child becomes an orphan and is adopted by
    the init process, whose process ID is 1.
*/

use nix::{
    libc::{self, sleep},
    unistd::{fork, getppid, write, ForkResult},
};

fn main() {
    let og_parent_pid = getppid();
    println!("The original parents pid is {og_parent_pid}\n");
    match unsafe { fork() } {
        Ok(ForkResult::Parent { .. }) => {
            unsafe { libc::_exit(0) };
        }
        Ok(ForkResult::Child) => {
            unsafe { sleep(3) }; // Give time for parent to terminate
            let parent_pid = getppid();
            write(
                std::io::stdout(),
                format!("Since I am ophaned my parents pid has become {parent_pid}\n")
                    .as_bytes(),
            )
            .ok();
            unsafe { libc::_exit(0) };
        }
        Err(_) => println!("Fork failed"),
    }
}
