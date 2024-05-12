use std::net::UdpSocket;
mod parser;
use parser::DnsResponse; 


use std::io::prelude::*;

fn create_message(name : &String) -> Vec<u8>{
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

fn read_message(res: Vec<u8>, name: String) -> Result<(), &'static str>   {


    let mut dns_response = DnsResponse::new(&res);

    
    dns_response.get_response_code(res[3], name.as_str())?;


    dns_response.parse_header(&res[0..12]); 
    
    let mut byte: usize = 12; 
    for _ in 0..dns_response.qd_count {

        let _q_name = dns_response.parse_qname(&res[12..res.len()], &mut byte);
        let (_q_type, _q_class) = dns_response.parse_question(&res[byte..res.len()]); 
        byte += 4;
 
    }

    if dns_response.an_count == 0 {
        return Err(Box::leak(format!("Dominio {} nao possui entrada NS", name ).into_boxed_str()));
    }
        
    for _ in 0..dns_response.an_count {
        let domain_name = dns_response.parse_answer(&res[byte..res.len()], &mut byte);

        if domain_name.is_none() {
            return Err(Box::leak(format!("Dominio {} nao possui entrada NS.", name ).into_boxed_str()));
        }

        println!("{} <> {}.{}", name, domain_name.unwrap(), name)
    }

    Ok(())
}

fn solve(name : String, server : String) ->  Result<(), &'static str> {

    let socket: UdpSocket = UdpSocket::bind("0.0.0.0:0")
        .expect("Couldn't bind to address");

    socket.set_read_timeout(Some(std::time::Duration::from_secs(2))).expect("set_read_timeout failed");

    let mut res: [u8; 511] = [0; 511];

    let mut count = 0;
    
    loop {

        socket.send_to(&create_message(&name) , format!("{}:53", server))
            .expect("Couldn't send message");


        if count > 3 {
            return Err(Box::leak(format!("Nao foi possivel coletar entrada NS para {}", name ).into_boxed_str()));
        }
    
        match socket.recv_from(&mut res) {
            Ok((size, _)) => {
                read_message(res[0..size].to_vec(), name)?;
                break; 
            },
            Err(_) => {
                count = count +  1;
            }
        }
    }


    //-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= DEBUG -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= 

    let mut file =  std::fs::File::create("debug_row_message.bin").unwrap();

    file.write_all(&res).unwrap();

    
    
    //-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= DEBUG -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= 


    Ok(())
}

fn main() {

    let args: Vec<String> = std::env::args().collect();


    let (name, server) = (args[1].clone(), args[2].clone());


    match solve(name, server) {
        Err(m) => println!("{}", m),
        _ => (),
    }

}


