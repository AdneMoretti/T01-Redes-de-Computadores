use std::net::UdpSocket;
mod parser;

pub use crate::parser::parser_dns;

//const secret = rand::thread_rng();


fn main() {
    let _id1: u8 = rand::random();
    let _id2: u8 = rand::random();


    let args: Vec<String> = std::env::args().collect();
    
    // let _id: u16 = transaction_id;
    // let _flags: u16 = 0x0100;
    // let _qd_count: u16 = 0x00001;
    // let _an_count: u16 = 0x0000;
    // let _ar_count: u16 = 0x0000;
    // let _ad_count: u16 = 0x0000;|
   //println!("{}", (((_id1 << 8)  as u16) | _id2 as u16)); 
   print!("ID = ");

   println! ("{} {}", _id1, _id2 );

    let mut buffer = vec![
        _id1, _id2, // id 
        0x01, 0x00, // Flags
        0x00, 0x01, // Question Count
        0x00, 0x00, // Answer
        0x00, 0x00, // Authority
        0x00, 0x00, // Additional
    ];

    // fga.unb.br
    // fga unb br

    //label.extend ( args[1].split(".").map(|x| (x.len(), x.as_bytes()) ));

    for label in args[1].split(".") {
        buffer.push(label.len() as u8);
        buffer.extend(label.as_bytes());
    }
    
    // Indicativo de Fim da Query
    buffer.push(0);
    
    buffer.extend([0x00, 0x02]);
    buffer.extend([0x00, 0x01]);

    let socket: UdpSocket = UdpSocket::bind("0.0.0.0:0").expect("Couldn't bind to address");

    socket.send_to(&buffer, format!("{}:53", args[2])).expect("Couldn't send message");

    let mut res: [u8; 255] = [0; 255];

    let (size, _) = socket.recv_from(&mut res).expect("Couldn't send message");

    println!("{}", size);
    println!("{}:53", args[2]);
    
    
    
    print!("res = ");
    //parser_dns::parse_response(&res[12+args[2].len()+5..size]); 
    
    for e in 12+args[1].len()+5..size{
        print!("{}: {} ", e, res[e] as char);
    }
    // socket.send();   
    
}

// Type NS 2 
// Class IN 1
// Query opcode = 0







