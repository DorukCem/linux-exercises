use std::
    os::fd::{AsFd, AsRawFd, RawFd}
;

use nix::{
    errno::Errno,
    libc::BUFSIZ,
    poll::PollTimeout,
    sys::{
        epoll::{Epoll, EpollCreateFlags, EpollEvent, EpollFlags},
        socket::{
            connect, recv, send, socket, AddressFamily, MsgFlags, SockFlag, SockType, UnixAddr,
        },
    },
    unistd::read,
};

//* This client can recieve both from a socket and standart in without waiting on either of them.
//* I have used two different non blocking reading techniques  

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

    // Create epoll
    let epoll = Epoll::new(EpollCreateFlags::empty()).expect("expect to create epoll");
    let event = EpollEvent::new(EpollFlags::EPOLLIN, std::io::stdin().as_raw_fd() as u64);
    epoll
    .add(std::io::stdin().as_fd(), event)
    .expect("Expected to crate event");
    let mut events = [EpollEvent::empty(); 1];

    // Since we do not want to send messages untill we get \n from stdin (kind of like read line)
    // we need to hold it in a buffer
    let mut stdin_buf = Vec::new();

    loop {
        // Get broadcasts from server
        let mut buf = [0; BUFSIZ as usize];
        //* Here we are recv'ing from socket with the flag MSG_DONTWAIT so that we can do other stuff if there is nothing to recieve
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
            Err(Errno::ECONNRESET) => {
                println!("Server disconnected.");
                break;
            }
            Err(e) => {
                eprintln!("Error receiving data: {:?}", e);
                break;
            }
        }
        
        //* Here we are using EPOLL to check if stdin is ready to read without blocking 
        let num_events = epoll
            .wait(&mut events, PollTimeout::ZERO)
            .expect("Expected to wait epoll");
        let mut buf = [0; BUFSIZ as usize]; // Testing a small buffer
        for i in 0..num_events {
            if events[i].data() as RawFd == std::io::stdin().as_raw_fd() {
                // Read available data from stdin
                let bytes_read = read(std::io::stdin().as_raw_fd(), &mut buf)
                    .expect("Expect to read from stdin");

                if bytes_read == 0 {
                    break;
                }
                // Since BUFSIZE is not guranteed to be big enough to hold every stdin message we have to append it to stdin buffer until it is ready to send  
                stdin_buf.extend_from_slice(&buf[..bytes_read]);
            }
        }

        // If message has \n then drain the buffer until \n and send it
        if let Some(pos) =stdin_buf.iter().position(|x| b'\n' == *x){
            let line: Vec<u8> = stdin_buf.drain(0..pos+1).into_iter().collect();
            send(socket_fd.as_raw_fd(), &line, MsgFlags::empty())
                .expect("Expected to send");
        }
    }

    println!("This client has exited");
}
