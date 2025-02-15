/*
    What happens to a child that terminates before its parent has had a chance to
    perform a wait()? The point here is that, although the child has finished its
    work, the parent should still be permitted to perform a wait() at some later
    time to determine how the child terminated. The kernel deals with this situa-
    tion by turning the child into a zombie. This means that most of the resources
    held by the child are released back to the system to be reused by other processes.
    The only part of the process that remains is an entry in the kernel’s process
    table recording (among other things) the child’s process ID, termination status,
    and resource usage statistics.
*/

use nix::{
    sys::wait::waitpid,
    unistd::{fork, ForkResult},
};
use std::{thread, time::Duration};

fn main() {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            println!(
                "Parent process: PID = {}, child PID = {}",
                std::process::id(),
                child
            );
            println!("Parent is sleeping, child becomes a zombie...");
            thread::sleep(Duration::from_secs(5)); // Parent delays, leaving child as zombie

            println!("Parent is now waiting for child...");
            match waitpid(child, None) {
                Ok(_) => println!("Child process {} reaped successfully!", child),
                Err(e) => eprintln!("Failed to wait for child: {}", e),
            }
        }
        Ok(ForkResult::Child) => {
            println!(
                "Child process: PID = {} is exiting immediately!",
                std::process::id()
            );
            std::process::exit(0); // Child exits but remains in the process table as a zombie until the parent waits
        }
        Err(_) => eprintln!("Fork failed"),
    }
}
