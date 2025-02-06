use nix::libc::{brk, sbrk};

//  sbrk() increments the program's data space by increment bytes.
//  Calling sbrk() with an increment of 0 can be used to find the current location of the program break.
//  On success, brk() returns zero.
//  On success, sbrk() returns the previous program break.

/// Allocates an uninilized value on the heap and returns a pointer to it
fn alloc<T: Sized>() -> *mut T {
    let mut size = std::mem::size_of::<T>();
    // increment untill power of 2 is power of 2
    while (size & (size - 1)) != 0 { 
        size += 1;
    }
    let pointer = unsafe { sbrk(size.try_into().unwrap()) } as *mut T;
    pointer
}

fn main() {
    let num1 = alloc::<[u16; 10]>();
    let num2 = alloc::<u64>();
    let num3 = alloc::<u64>();
    unsafe { *num1 = [0; 10] };
    unsafe { *num2 = u64::MAX };
    unsafe { *num3 = 32 };
    let val = unsafe { *num2 };
    println!("{}", val);
}
