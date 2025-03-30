use std::{os::fd::AsRawFd, sync::mpsc::channel};

use nix::{
    errno::Errno,
    libc::BUFSIZ,
    sys::socket::{
        connect, recv, send, socket, AddressFamily, MsgFlags, SockFlag, SockType, UnixAddr,
    },
};

fn main() {
    let socket_fd = socket(
        AddressFamily::Unix,
        SockType::Stream,
        SockFlag::empty(),
        None,
    )
    .expect("Expected to open socket");

    let path = "/tmp/socket_exercise.sock";

    connect(
        socket_fd.as_raw_fd(),
        &UnixAddr::new(path).expect("Expected to crate unix address"),
    )
    .expect("Expected to connect to server");

    let (sender, reciever) = channel();
    std::thread::spawn(move || loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        sender.send(buffer).unwrap();
    });

    loop {
        // Get broadcasts from server
        let mut buf = [0; BUFSIZ as usize];
        // I also need this to be non blocking
        match recv(socket_fd.as_raw_fd(), &mut buf, MsgFlags::MSG_DONTWAIT) {
            Ok(bytes_read) if bytes_read > 0 => {
                println!(
                    "Receved broadcast: {}",
                    String::from_utf8_lossy(&buf[..bytes_read])
                )
            }
            Ok(_) => {
                println!("Connection has closed");
                break;
            }
            #[allow(unreachable_patterns)]
            Err(Errno::EAGAIN) | Err(Errno::EWOULDBLOCK) => {
                // No data available, continue looping
            }
            Err(e) => {
                eprintln!("Error receiving data: {:?}", e);
                break;
            }
        }

        if let Ok(msg) = reciever.try_recv(){
            let msg = msg.trim();
            if msg == "exit" {
                break;
            }

            send(
                socket_fd.as_raw_fd(),
                msg.as_bytes(),
                MsgFlags::empty(),
            )
            .expect("Expected to send");
        }
        
    }

    println!("This client has exited");
}
