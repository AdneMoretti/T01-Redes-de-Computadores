
pub struct DnsHeader {
    id: u16, 
    flags: u16, 
    qd_count: u16, 
    an_count: u16, 
    ar_count: u16    
}

pub mod parser_dns {
    use super::DnsHeader;

    pub fn parse_response(res: &[u8]){
        let id1: u8 = res[0]; 
        let id2: u8 = res[1]; 
        //let header: DnsHeader::new();

        //let display_result = res
        //String::from_utf8(display_result).unwrap();
        let record_type: u16 = ((id1 as u16) << 8) | (id2 as u16);
        self::parse_qname(res);

        
    }

    pub fn parse_flags(res: &[u8]){
        
    }

    pub fn parse_qname(res: &[u8]){
        let mut qname = String::from("");
        let mut pos: usize = 12; 
        println!("PARSE_QNAME"); 
        loop {
            let size: u8 = res[pos]; 
            
            if res[pos] == 0 {
                break; 
            }
            println!("{}", size);
    
            let my_vec: Vec<u8> = res[13..pos+(size as usize)+1].to_vec();

            qname.push_str(&(String::from_utf8(my_vec)).unwrap());
            println!("{}", qname);
            pos+=size as usize;

        }

    }

}