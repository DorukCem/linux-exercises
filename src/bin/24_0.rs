/* Testing fork */
/* Note: fork is copy on write meaning that the actual copy is only created when a write to the data occurs. This prevents waste */

use nix::{libc, sys::wait::waitpid, unistd::{fork, write, ForkResult}};

fn main() {
    match unsafe{fork()} {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("Continuing execution in parent process, new child has pid: {}", child);
            waitpid(child, None).unwrap(); // wait untill child exits
        }
        Ok(ForkResult::Child) => {
            // Unsafe to use `println!` (or `unwrap`) here. See Safety.
            write(std::io::stdout(), "I'm a new child process\n".as_bytes()).ok();
            unsafe { libc::_exit(0) };
        }
        Err(_) => println!("Fork failed"),
    }
}