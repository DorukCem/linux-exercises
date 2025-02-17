use std::{ffi::c_void, ptr};

use nix::libc::{pthread_create, pthread_join};

extern "C" fn thread_func(_arg: *mut c_void) -> *mut c_void {
    println!("Hello from the new thread!");
    let return_value = Box::new(42);
    Box::into_raw(return_value) as *mut c_void
}

fn main() {
    let mut thread = unsafe { std::mem::zeroed() };
    let mut return_val: *mut c_void = ptr::null_mut();

    unsafe {
        // Create a new thread
        let result = pthread_create(&mut thread, ptr::null(), thread_func, ptr::null_mut());

        if result != 0 {
            eprintln!("Failed to create thread: {}", result);
            return;
        }
        println!("Thread created successfully.");

        // Wait for the thread to finish
        pthread_join(thread, &mut return_val);
        let value = return_val as *const i32;
        println!("Thread has returned value {}", *value);
    }
}
