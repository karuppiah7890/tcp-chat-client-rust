use std::thread;
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:8080").unwrap();

    let write_thread = thread::spawn(|| {
        handle_write(stream);        
    });

    let stream = TcpStream::connect("127.0.0.1:8080").unwrap();

    let read_thread = thread::spawn(|| {
        handle_read(stream);        
    });

    read_thread.join().unwrap();
    write_thread.join().unwrap();
}

fn handle_read(mut stream: TcpStream) {
    loop {
        let mut buffer = [0; 512];    
        let size = stream.read(&mut buffer).unwrap();
        if size == 0 {
            continue;
        }
        let message = String::from_utf8_lossy(&buffer[..]);
        println!("\n\nserver : {}", message);            
    }
}

fn handle_write(mut stream: TcpStream) {
    loop {
        let mut message = String::new();
        io::stdin().read_line(&mut message)
                .expect("Failed to read line");

        stream.write(message.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
