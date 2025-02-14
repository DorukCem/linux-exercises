/*
    Write a program that registers a signal handler for SIGINT (Ctrl+C).
    When the signal is received, print "Caught SIGINT" and continue execution.
*/

use std::{
    os::fd::BorrowedFd,
    sync::atomic::{AtomicBool, Ordering},
};

use nix::{
    libc::{c_int, STDOUT_FILENO},
    sys::signal::{signal, SigHandler, Signal},
    unistd::write,
};

static SIGNALED: AtomicBool = AtomicBool::new(false);

extern "C" fn handle_sigint(signal: c_int) {
    let signal = Signal::try_from(signal).unwrap();
    SIGNALED.store(signal == Signal::SIGINT, Ordering::Relaxed);
    unsafe {
        write(
            BorrowedFd::borrow_raw(STDOUT_FILENO),
            "Cought one! \n".as_bytes(),
        )
        .unwrap(); // Write is safe to use in signal handlers
    }
}

fn main() {
    let handler = SigHandler::Handler(handle_sigint);
    unsafe { signal(Signal::SIGINT, handler) }.unwrap();
    loop {}
}
