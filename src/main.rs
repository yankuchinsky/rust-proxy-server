use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
use curl::easy::Easy;
use std::io::{stdout, Write};

fn main() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
  let request_url = "http://127.0.0.1:3000/";

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    handle_connection(stream, request_url);
  }
}

fn handle_connection(mut stream: TcpStream, request_url: &str) {
  let mut buffer = [0; 1024];
  let url = request_url.to_string();

  stream.read(&mut buffer).unwrap();

  let b_content = str::from_utf8(&buffer).unwrap().to_string();

  println!("{}", b_content);
  
  let lines = parse_lines(&b_content);
  let first_line = &lines[0];
  let last_line = &lines[lines.len() - 1];
  let mut easy = Easy::new();

  
  let method = &first_line[..first_line.find(" ").unwrap()];
  
  let mut data = last_line.as_bytes();

  easy.url(&url).unwrap();



  if method == "GET" {
    easy.write_function(|data| {
      stdout().write_all(data).unwrap();
      Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
    
  } else {
    easy.post(true).unwrap();

    easy.post_field_size(data.len() as u64).unwrap();

    let mut transfer = easy.transfer();
    transfer.read_function(|buf| {
        Ok(data.read(buf).unwrap_or(0))
    }).unwrap();
    transfer.perform().unwrap();
  }


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

fn parse_lines(content_string: &String) -> Vec<String>{
  let spl:Vec<&str> = (&content_string).split("\n").collect();
  let mut num: i32 = 0;

  // Get content length
  for line in spl.iter() {
    let str = line.to_string();

    if str.starts_with("Content-Length") {
      num = (&str[str.find(" ").unwrap()..]).trim().parse::<i32>().unwrap();
    }
  }

  let mut res: Vec<String> = Vec::new();
  let mut body: Vec<String> = Vec::new();
  let size = spl.len() as i32;

  // combine lines
  for i in 0..size {
    let ln = spl[i as usize].to_string();

    if i < num {
      res.push(ln);
    } else {
      body.push(ln);
    } 
  }

  if body.len() > 0 {
    res.push(body.join(""));
  }

  return res;
}
