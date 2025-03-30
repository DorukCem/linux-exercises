use std::{
    os::fd::AsRawFd,
    path::Path,
    sync::{Arc, Mutex},
};

use nix::{
    libc::BUFSIZ,
    sys::socket::{
        accept, bind, listen, recv, send, socket, AddressFamily, Backlog, MsgFlags, SockFlag,
        SockType, UnixAddr,
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

    let path = Path::new("/tmp/socket_exercise.sock");
    if path.exists() {
        std::fs::remove_file(path).expect("Expected to remove existing socket file");
    }

    bind(
        socket_fd.as_raw_fd(),
        &UnixAddr::new(path).expect("Expected to crate unix address"),
    )
    .expect("Expected socket to bind");

    listen(&socket_fd, Backlog::MAXCONN).expect("Expected to listen to socket");

    println!("Started server");

    let clients = Arc::new(Mutex::new(Vec::new()));

    loop {
        /*
            The key point to understand about accept() is that it creates a new socket, and it is
            this new socket that is connected to the peer socket that performed the connect(). A
            file descriptor for the connected socket is returned as the function result of the
            accept() call. The listening socket (socket_fd) remains open, and can be used to accept
            further connections.
        */
        let client_socket = accept(socket_fd.as_raw_fd()).expect("Expected to accept connection");

        {
            let mut clients = clients.lock().expect("Expected to get the mutex");
            clients.push(client_socket);
        }

        let clients = Arc::clone(&clients);
        std::thread::spawn(move || {
            println!("New client connected");
            loop {
                // Recieve msg
                let mut msg = [0; BUFSIZ as usize];
                if let Ok(bytes_read) = recv(client_socket, &mut msg, MsgFlags::empty()) {
                    if bytes_read == 0 {
                        break;
                    }

                    // Broadcast msg to all connected clients
                    let clients = clients.lock().expect("Expected to get mutex");
                    for client_fd in clients.iter() {
                        send(client_fd.as_raw_fd(), &msg[..bytes_read], MsgFlags::empty())
                            .expect("Expected to send");
                    }
                } else {
                    break;
                }
            }

            // If this thread is terminated we have to remove this connection from clients
            let mut clients = clients.lock().expect("Expected to get mutex");
            let idx = clients
                .iter()
                .position(|x| *x == client_socket)
                .expect("Expected to find this fd in clients") as usize;
            clients.remove(idx);
            println!("Removed a client");
        });
    }
}
