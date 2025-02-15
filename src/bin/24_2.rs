/*
    We can combine the use of fork() and wait() to control the memory footprint of a
    process.

    After executing this code, we know that the
    memory footprint of the parent is unchanged from the point before func() was
    called, since all possible changes will have occurred in the child process.

    If we know that func() causes memory leaks or excessive fragmentation of the
    heap, this technique eliminates the problem

    Suppose that we have some algorithm that performs memory allocation while
    doing a tree analysis (for example, a game program that analyzes a range of
    possible moves and their responses). We could code such a program to make
    calls to free() to deallocate all of the allocated memory. However, in some cases,
    it is simpler to employ the technique we describe here in order to allow us to
    backtrack, leaving the caller (the parent) with its original memory footprint
    unchanged.
*/

use nix::{
    libc::exit,
    sys::wait::waitpid,
    sys::wait::WaitStatus,
    unistd::{fork, ForkResult},
};

fn some_leaky_function() -> i32 {
    // Simulate a function that may cause memory leaks or fragmentation
    42
}

fn main() {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            // Wait for the child process to exit and retrieve its status
            match waitpid(child, None) {
                Ok(WaitStatus::Exited(_, status)) => {
                    println!("Child calculatd function result as: {}", status);
                }
                _ => println!("Child did not exit normally"),
            }
        }
        Ok(ForkResult::Child) => {
            let result = some_leaky_function();
            unsafe { exit(result) };
        }
        Err(_) => println!("Fork failed"),
    }
}
