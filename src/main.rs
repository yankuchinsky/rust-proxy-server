use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;

fn main() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    handle_connection(stream);
  }
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 1024];

  stream.read(&mut buffer).unwrap();

  let get = b"GET / HTTP/1.1\r\n";

  let b_content = str::from_utf8(&buffer).unwrap().to_string();

  println!("{}", b_content);

  let request_type = parse_request_type(&b_content);
  let url = parse_url(&b_content);

  println!("{}", url);


  let res = reqwest::blocking::get("")?;

  let contents = "conents";

  let response = format!(
      "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
      contents.len(),
      contents
  );

  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
  
}

fn parse_request_type(content_string: &String) -> String {
  let index = content_string.find(" ").unwrap();

  return (&content_string[..index]).to_string();
}

fn parse_url(content_string: &String) -> String{
  let start_index = content_string.find("/").unwrap();  
  let end_index = content_string.find("HTTP").unwrap();
 
  return (&content_string[start_index..end_index]).to_string();
}