use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, fs
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878")
    .unwrap();


    for stream in listener.incoming(){
        let stream = stream.unwrap();
        handle_connnection(stream);
    }
}

fn handle_connnection(mut stream: TcpStream) {
    let buf_reader  = BufReader::new(&mut stream);
    let req_line = &buf_reader
    .lines()
    .next()
    .unwrap()
    .unwrap();


    let (status_line, file_name) = if req_line == "GET / HTTP/1.1"{
        ("HTTP/1.1 200 OK", "hello.html")
    }else{
        ("HTTP/1.1 404 PAGE NOT FOUND", "404.html")
    };

    let content = fs::read_to_string(file_name).unwrap();
    let length = content.len();

    let res = format!( "{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

    stream.write_all(res.as_bytes()).unwrap();

}