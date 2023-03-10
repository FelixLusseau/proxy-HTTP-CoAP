use coap_lite::{CoapRequest, RequestType as Method};
use std::net::{SocketAddr, UdpSocket};
use std::str;

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    // Web server waiting for requests on port 8080
    let listener = TcpListener::bind("10.42.0.100:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Read the request
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let mut path = "/";
    let mut status_line = "HTTP/1.1 404 NOT FOUND";

    if request_line == "GET /temperature HTTP/1.1" {
        status_line = "HTTP/1.1 200 OK";
        path = "/temperature";
    } else if request_line == "GET /humidity HTTP/1.1" {
        status_line = "HTTP/1.1 200 OK";
        path = "/humidity";
    } else if request_line == "GET /light HTTP/1.1" {
        status_line = "HTTP/1.1 200 OK";
        path = "/light";
    } else {
        // Return 404 if the path is not found upwards
        let contents = "404 NOT FOUND\r\n";
        let length = contents.len();
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}\r\n");
        stream.write_all(response.as_bytes()).unwrap();
        return;
    }

    // Send the request to the CoAP server on the Arduino
    let mut request: CoapRequest<SocketAddr> = CoapRequest::new();

    request.set_method(Method::Get);
    request.set_path(path);

    let socket = UdpSocket::bind("10.43.0.1:0").unwrap();

    let packet = request.message.to_bytes().unwrap();
    socket
        .send_to(&packet[..], "10.43.0.217:5683")
        .expect("Could not send the data");

    // Receive the response from the CoAP server
    let mut buf = [0; 1024];
    let mut contents = "";
    match socket.recv(&mut buf) {
        Ok(received) => {
            contents = str::from_utf8(&buf[8..received]).unwrap(); // Save the response in a variable
            println!(
                "received {received} bytes {:?}",
                str::from_utf8(&buf[8..received]).unwrap()
            )
        }
        Err(e) => println!("recv function failed: {e:?}"),
    }

    // Send the HTTP response to the client
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}\r\n\r\n");

    stream.write_all(response.as_bytes()).unwrap();
}
