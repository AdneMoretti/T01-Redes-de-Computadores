use std::net::UdpSocket;

//const secret = rand::thread_rng();

struct DnsHeader {
    id: u16, 
    flags: u16, 
    qd_count: u16, 
    an_count: u16, 
    ar_count: u16    
}

/*
HEADER     		OPCODE=0	
QUESTION		QNAME="unb.br", QCLASS=01, QTYPE="NS"
ANSWER          <empty>  0x0000
AUTHORITY       <empty>  0x0000
ADDITIONAL      <empty>  0x0000
*/
struct DnsQuestion {
    q_name: u16,
    q_type: u16,
    q_class: u16
}

fn main() {
    let transaction_id: u16 = rand::random();
    println!("{}", transaction_id);
    let args: Vec<String> = std::env::args().collect();

    
    let id: u16 = transaction_id;
    let flags: u16 = 
    let qd_count: u16;
    let an_count: u16;
    let ar_count: u16;

    let c = vec![1, 2, 3, 4];
 


    let socket: UdpSocket = UdpSocket::bind("0.0.0.0:0").expect("Couldn't bind to address");
    socket.connect(format!("{}:53", args[2])).expect("Couldn't connect to address"); 

    println!("{}:53", args[2]);

    // socket.send();   
    
}

// Type NS 2 
// Class IN 1
// Query opcode = 0







