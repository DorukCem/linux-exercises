use nix::sys::inotify::{AddWatchFlags, InitFlags, Inotify};

/*
    Write a program that logs all file creations, deletions, and renames under the
    directory named in its command-line argument.
*/
fn main() {
    // We create a new inotify instance.
    let instance = Inotify::init(InitFlags::empty()).unwrap();

    // We add a new watch on directory "test" for all events.
    let _wd = instance
        .add_watch(".", AddWatchFlags::IN_ALL_EVENTS)
        .unwrap();

    loop {
        // We read from our inotify instance for events.
        let events = instance.read_events().unwrap();
        println!("Events: {:?}", events);
    }
}
