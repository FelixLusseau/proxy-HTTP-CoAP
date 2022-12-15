# Proxy HTTP-CoAP

This is a simple proxy in Rust that forwards HTTP requests from internet on a Raspberry Pi 3 B to a CoAP server on a Arduino MKR 1010 through 2 ethernet subnetworks.

<a href="https://www.rust-lang.org" target="_blank" rel="noreferrer"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/480px-Rust_programming_language_black_logo.svg.png" alt="rust" width="40" height="40"/></a>
<a href="https://www.raspberrypi.com" target="_blank" rel="noreferrer"> <img src="https://www.raspberrypi.com/app/uploads/2022/02/COLOUR-Raspberry-Pi-Symbol-Registered.png" alt="raspberry" width="40" height="40"/> </a> 
<a href="https://www.arduino.cc" target="_blank" rel="noreferrer"> <img src="https://cdn.worldvectorlogo.com/logos/arduino-1.svg" alt="arduino" width="40" height="40"/> </a> 

Set the IP addresses of the Raspberry Pi and the Arduino in the `src/main.rs` file.
Use `cargo run` to compile and run the proxy.