use std::net::UdpSocket;
mod parser;
use parser::DnsResponse; 

// pub use crate::parser::DnsResponse;

//const secret = rand::thread_rng();

fn send(server : String, name : String, socket: &UdpSocket) {
    println!("{}", name); 
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
    buffer.extend([0x00, 0x02]);
    buffer.extend([0x00, 0x01]);
    

    socket.send_to(&buffer, format!("{}:53", server)).expect("Couldn't send message");
    
}

fn receive(socket: &UdpSocket, name: String, server : String,) -> Vec<u8>  {
    let mut res: [u8; 255] = [0; 255];
    
    let (size, _) = socket.recv_from(& mut res).expect("Couldn't send message");
    
    let mut dns_response = DnsResponse {
        id: 0, 
        flags: 0, 
        qd_count: 0, 
        an_count: 0, 
        ns_count: 0, 
        ar_count: 0 
    };

    match dns_response.get_response_code(res[3], name.as_str()){
        Ok(()) => println!("funcionou"), 
        Err(e) => println!("{}", e)
    }

    dns_response.parse_header(&res[0..12]); 
    
    let mut byte: usize = 12; 
    for _ in 0..dns_response.qd_count {
        let q_name = dns_response.parse_qname(&res[12..size], &byte);
        let (q_type, q_class) = dns_response.parse_question(&res[byte..size]); 
        println!("{} : {}, { } ", q_name, q_type, q_class);
    }
        
    for _ in 0..dns_response.an_count {

    }

    for _ in 0..dns_response.ns_count {

    }

    for _ in 0..dns_response.ar_count {

    }

    for e in 0..size {
        print!("{}: {} ", e, res[e]);
    }
    return res[0..size].to_vec()
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  
  
  let socket: UdpSocket = UdpSocket::bind("0.0.0.0:0").expect("Couldn't bind to address");

  send(args[2].clone(), args[1].clone(), &socket);

  let mut res: Vec<u8> = receive(&socket, args[1].clone(), args[2].clone()); 

    
}

// Type NS 2 
// Class IN 1
// Query opcode = 0







