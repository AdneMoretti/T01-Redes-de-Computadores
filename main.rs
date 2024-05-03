use std::net::UdpSocket

fn main() {

    let args: Vec<String> = std::env::args().collect();


    let socket = UdpSocket::bind("0.0.0.0:0").expect("Couldn't bind to address");

    socket.connect("{}:53", args[2]).expect("Couldn´t connect to address"); 

    socket.send();
    
}