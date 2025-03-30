/* Sync with mutex */

#![allow(static_mut_refs)]
use std::{
    ffi::c_void,
    ptr::{self, addr_of_mut},
};

use nix::libc::{
    pthread_create, pthread_join, pthread_mutex_lock, pthread_mutex_t, pthread_mutex_unlock,
    PTHREAD_MUTEX_INITIALIZER,
};
static mut NUM: u64 = 0;
static mut MUTEX: pthread_mutex_t = PTHREAD_MUTEX_INITIALIZER;

extern "C" fn thread_func(_arg: *mut c_void) -> *mut c_void {
    for _ in 0..100000 {
        unsafe { pthread_mutex_lock(addr_of_mut!(MUTEX)) };
        unsafe { NUM += 1 };
        unsafe { pthread_mutex_unlock(addr_of_mut!(MUTEX)) };
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
