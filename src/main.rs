use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn show_correct_page(mut stream: TcpStream, status_line: &str , html_file_name: &str){
    let contents = fs::read_to_string(html_file_name).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        show_correct_page(stream, "HTTP/1.1 200 OK", "hello.html")
    }
    else {
        show_correct_page(stream, "HTTP/1.1 404 NOT FOUND", "404.html")
    }
}
