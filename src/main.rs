use std::io::{Result, Write};
use std::thread;
use std::net::{Ipv4Addr, TcpListener, TcpStream};



fn handle_client(mut stream: TcpStream) -> Result<()> {
    stream.write(b"Hello World\r\n")?;
    Ok(())
}


fn main() {
    let host = Ipv4Addr::new(127, 0, 0, 1);
    let port = 9000;
    let listener = TcpListener::bind((host, port)).unwrap();

    for stream in listener.incoming() {
        match stream {
            Err(e) => println!("Failed: {}", e),
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
        }
    }
}
