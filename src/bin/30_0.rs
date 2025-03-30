/* This code creates a race condition and therefore at the end num is smaller then it should be */
#![allow(static_mut_refs)]

use std::{ffi::c_void, ptr};

use nix::libc::{pthread_create, pthread_join};

static mut NUM: u64 = 0;

extern "C" fn thread_func(_arg: *mut c_void) -> *mut c_void {
    for _ in 0..100000 {
        unsafe { NUM += 1 };
    }
    ptr::null_mut::<c_void>()
}

fn main() {
    let mut t1 = unsafe { std::mem::zeroed() };
    let mut t2 = unsafe { std::mem::zeroed() };

    unsafe {
        // Create a new thread
        let _ = pthread_create(&mut t1, ptr::null(), thread_func, ptr::null_mut());
        let _ = pthread_create(&mut t2, ptr::null(), thread_func, ptr::null_mut());

        // Wait for the thread to finish
        pthread_join(t1, ptr::null_mut());
        pthread_join(t2, ptr::null_mut());
        println!("{}", NUM);
    }
}
