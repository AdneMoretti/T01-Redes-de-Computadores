use std::net::UdpSocket;
mod parser;
use parser::DnsResponse; 


use std::io::prelude::*;

fn create_message(name : &String) -> Vec<u8>{
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
    
    buffer
    
}

fn read_message(res: Vec<u8>, name: String)   {


    let mut dns_response = DnsResponse::new(&res);
    

    match dns_response.get_response_code(res[3], name.as_str()){
        Ok(()) => {
            println!("funcionou")
        }, 
        Err(e) => println!("{}", e)
    }

    dns_response.parse_header(&res[0..12]); 
    
    let byte: usize = 12; 
    for _ in 0..dns_response.qd_count {
        let q_name = dns_response.parse_qname(&res[12..res.len()], &byte);
        let (q_type, q_class) = dns_response.parse_question(&res[byte..res.len()]); 
        println!("{} : {}, {} ", q_name, q_type, q_class);  
    }
        
    for _ in 0..dns_response.an_count {
        
    }

    for _ in 0..dns_response.ns_count {

    }

    for _ in 0..dns_response.ar_count {

    }


}

fn main() {

    let args: Vec<String> = std::env::args().collect();


    let (name, server) = (args[1].clone(), args[2].clone());


    let socket: UdpSocket = UdpSocket::bind("0.0.0.0:0")
        .expect("Couldn't bind to address");


    socket.send_to(&create_message(&name) , format!("{}:53", server))
        .expect("Couldn't send message");


    let mut res: [u8; 255] = [0; 255];
    let (size, _) = socket.recv_from(&mut res)
        .expect("Couldn't recv message");

    read_message(res[0..size].to_vec(), name); 


    //-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= DEBUG -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= 

    let mut file =  std::fs::File::create("debug_row_message.bin").unwrap();

    file.write_all(&res).unwrap();

    for e in 0..res.len() {
        print!("{} ", res[e]);
    }
    
    //-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= DEBUG -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= 
}


