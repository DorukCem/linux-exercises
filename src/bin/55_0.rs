use nix::fcntl::{Flock, FlockArg};
use std::io::Write;

fn main() {
    let file = std::fs::File::open("test/a.txt").unwrap();

    let mut lock = match Flock::lock(file, FlockArg::LockExclusive) {
        Ok(l) => l,
        Err(_) => return,
    };

    let data = "Foo bar";
    lock.write(data.as_bytes()).unwrap();
    lock.sync_data().unwrap();

    // Lock is unlocked when dropped
}
