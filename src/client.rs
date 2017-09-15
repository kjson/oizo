use std::net::TcpStream;
use std::io::{Write, BufWriter};


fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:9000").unwrap();
    let mut writer = BufWriter::new(&mut stream);
    writer.write(b"Test.").unwrap();
}
