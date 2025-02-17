/* 
    Threads can have their own static data that is not 
    shared with other threads, this data can then be accesses trough 
    consecitve function calls (just as a regular static variable). 
    However this static data is only local to the thread and cannot be acceses by other threads.
*/

use std::cell::RefCell;
use std::thread;
use std::time::Duration;

// Define a thread-local storage variable
thread_local! {
    static THREAD_COUNTER: RefCell<u32> = RefCell::new(0);
}

// Helper function to get current thread name or ID
fn get_thread_identifier() -> String {
    thread::current()
        .name()
        .unwrap_or(&format!("{:?}", thread::current().id()))
        .to_string()
}

fn increment_counter() {
    THREAD_COUNTER.with(|counter| {
        *counter.borrow_mut() += 1;
        println!(
            "Thread {}: Counter value is {}",
            get_thread_identifier(),
            counter.borrow()
        );
    });
}

fn main() {
    // Create a vector to store our thread handles
    let mut handles = vec![];

    // Spawn three threads
    for i in 1..=3 {
        let handle = thread::Builder::new()
            .name(format!("Worker-{}", i))
            .spawn(move || {
                println!("Started thread {}", get_thread_identifier());

                // Each thread will increment its own counter 3 times
                for _ in 0..3 {
                    increment_counter();
                    thread::sleep(Duration::from_millis(100));
                }

                // Final counter value for this thread
                THREAD_COUNTER.with(|counter| {
                    println!(
                        "Thread {} finishing with counter value {}",
                        get_thread_identifier(),
                        counter.borrow()
                    );
                });
            })
            .unwrap();

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Main thread's counter remains unaffected
    THREAD_COUNTER.with(|counter| {
        println!(
            "Main thread {} counter value is {}",
            get_thread_identifier(),
            counter.borrow()
        );
    });
}
