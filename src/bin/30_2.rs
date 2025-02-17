/* Sync with cond variables */

use nix::libc::{
    pthread_cond_signal, pthread_cond_wait, pthread_create, pthread_join, pthread_mutex_lock,
    pthread_mutex_t, pthread_mutex_unlock, PTHREAD_COND_INITIALIZER, PTHREAD_MUTEX_INITIALIZER,
};
use std::{
    ffi::c_void,
    ptr::{self},
};

use std::boxed::Box;

use nix::libc::pthread_cond_t;

// Shared data structure
struct SharedData {
    value_ready: bool,
    value: i32,
    mutex: pthread_mutex_t,
    cond: pthread_cond_t,
}

extern "C" fn producer(arg: *mut c_void) -> *mut c_void {
    let data = unsafe { &mut *(arg as *mut SharedData) };
    unsafe {
        pthread_mutex_lock(&mut data.mutex);
        // Set the value and mark it as ready
        data.value = 42;
        data.value_ready = true;
        println!("Producer: Value set to {}", data.value);
        // Signal the main thread
        pthread_cond_signal(&mut data.cond);
        pthread_mutex_unlock(&mut data.mutex);
    }

    ptr::null_mut()
}

fn main() {
    let mut thread = unsafe { std::mem::zeroed() }; // Initialize thread ID

    // Initialize shared data
    let shared_data = Box::new(SharedData {
        value_ready: false,
        value: 0,
        mutex: PTHREAD_MUTEX_INITIALIZER,
        cond: PTHREAD_COND_INITIALIZER,
    });

    let shared_ptr = Box::into_raw(shared_data);

    unsafe {
        // Create a producer thread
        let result = pthread_create(
            &mut thread,
            ptr::null(),
            producer,
            shared_ptr as *mut c_void,
        );
        if result != 0 {
            eprintln!("Failed to create thread: {}", result);
            return;
        }
        let shared_data = &mut *shared_ptr;
        
        //Since pthread_cond_wait() unlocks the mutex while waiting, the lock becomes available for the producer thread.
        pthread_mutex_lock(&mut shared_data.mutex);
        // Wait until the value is ready
        while !shared_data.value_ready {
            pthread_cond_wait(&mut shared_data.cond, &mut shared_data.mutex);
        }
        println!("Main: Received value: {}", shared_data.value);
        pthread_mutex_unlock(&mut shared_data.mutex);
        // Wait for the producer thread to finish
        pthread_join(thread, ptr::null_mut());
        // Clean up
        let _ = Box::from_raw(shared_ptr);
    }
}
