/*
    Signal Masking with sigprocmask()

    Block SIGUSR1 before entering a critical section.
    Unblock it after leaving the critical section.
    Test it by sending SIGUSR1 while the program is running.
*/

use std::os::fd::BorrowedFd;

use nix::{
    libc::{c_int, STDOUT_FILENO},
    sys::signal::{signal, sigprocmask, SigHandler, SigSet, SigmaskHow, Signal},
    unistd::{sleep, write},
};
use std::process;

extern "C" fn handle_sigusr1(_signal: c_int) {
    unsafe {
        write(
            BorrowedFd::borrow_raw(STDOUT_FILENO),
            "Processesing sig user one! \n".as_bytes(),
        )
        .unwrap(); // Write is safe to use in signal handlers
    }
}

fn main() {
    println!("My pid is {}", process::id());

    let handler = SigHandler::Handler(handle_sigusr1);
    unsafe { signal(Signal::SIGUSR1, handler) }.unwrap();

    let mut signals_to_block = SigSet::empty();
    signals_to_block.add(Signal::SIGUSR1);

    let mut old_mask = SigSet::empty(); // Store old mask

    loop {
        // Block SIGUSR1 and store previous mask
        sigprocmask(
            SigmaskHow::SIG_BLOCK,
            Some(&signals_to_block),
            Some(&mut old_mask),
        )
        .unwrap();
        println!("Entering critical section...");
        sleep(5);
        println!("Exiting critical section...");

        // Restore previous signal mask
        sigprocmask(SigmaskHow::SIG_SETMASK, Some(&old_mask), None).unwrap();

        // Sleep to allow signal handling
        sleep(1);
    }
}
