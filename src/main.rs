use std::{env, thread, net};
use std::io::Write;

fn main() {
    let qotd = match env::args().skip(1).next() {
        Some(v) => v,
        None => {
            println!("Please give a QotD when calling the program, like so:");
            println!("{} [QotD]", env::args().next().unwrap());
            return;
        }
    };

    println!("Using QotD: '{}'", qotd);
    let mut qotd_buf = qotd.as_bytes().to_vec();
    qotd_buf.push(0); // null-terminate string

    let buf1 = qotd_buf.clone();
    thread::spawn(move || {
        let qotd_buf = buf1;
        let socket = net::UdpSocket::bind("0.0.0.0:17").unwrap();
        println!("UDP socket bound");
        let mut buf = [0; 64];
        loop {
            match socket.recv_from(&mut buf) {
                Ok((size, addr)) => {
                    println!("Received UDP datagram (size {}) from {}", size, addr);
                    socket.send_to(&qotd_buf, addr).unwrap();
                },
                Err(_) => (),
            }
        }
    });

    thread::spawn(move || {
        let socket = net::TcpListener::bind("0.0.0.0:17").unwrap();
        println!("TCP listener bound");
        let mut buf = [0; 64];
        loop {
            match socket.accept() {
                Ok((mut stream, addr)) => {
                    println!("Received TCP connection from {}", addr);
                    stream.write_all(&qotd_buf).unwrap();
                    stream.shutdown(net::Shutdown::Both).unwrap()
                },
                Err(_) => (),
            }
        }
    }).join().unwrap();
}
