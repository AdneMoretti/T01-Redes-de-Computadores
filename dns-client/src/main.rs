use std::net::UdpSocket;
mod parser;

pub use crate::parser::parser_dns;

//const secret = rand::thread_rng();

fn send(server : String, name : String ) -> Vec<u8> {

    let _id1: u8 = rand::random();
    let _id2: u8 = rand::random();

    let mut buffer = vec![
        _id1, _id2, // id 
        0x01, 0x00, // Flags
        0x00, 0x01, // Question Count
        0x00, 0x00, // Answer
        0x00, 0x00, // Authority
        0x00, 0x00, // Additional
    ];


    for label in name.split(".") {
        buffer.push(label.len() as u8);
        buffer.extend(label.as_bytes());
    }
    
    // Indicativo de Fim da Query
    buffer.push(0);
    

    let socket: UdpSocket = UdpSocket::bind("0.0.0.0:0").expect("Couldn't bind to address");

    socket.send_to(&buffer, format!("{}:53", server)).expect("Couldn't send message");

    let mut res: [u8; 255] = [0; 255];

    let (size, _) = socket.recv_from(&mut res).expect("Couldn't send message");


    return res[0..size].to_vec()
}


fn main() {
  let args: Vec<String> = std::env::args().collect();

  let res: Vec<u8> = send(args[2], args[1]);

  parser_dns::parse_response(&res); 
  
  for (i, e) in res.iter().enumerate() {
      print!("{}: {} ", i, e);
  }
  // socket.send();   
    
}

// Type NS 2 
// Class IN 1
// Query opcode = 0







