
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
        //let header: DnsHeader::new();

        //let display_result = res
        //String::from_utf8(display_result).unwrap();
        self::parse_id(res[0], res[1]); 
        self::parse_qname(res);
        // header: DnsHeader::new();

        //let display_result = res
        //String::from_utf8(display_result).unwrap();
        parse_flags(res);
        
    }
    
    pub fn parse_id(id_1: u8, id_2: u8) -> u16{
        let record_type: u16 = ((id_1 as u16) << 8) | (id_2 as u16);
        return record_type; 
    }

    pub fn parse_flags(res: &[u8]){
        let flags: u16 = ((res[2] as u16) << 8) + (res[3] as u16);
        println!("FLAGS {}", flags);
    }

    // Criei essa função aqui para dar o parse no name, mas coloquei do primeiro name, temos que alterar para pegar o qname que vem no answer
    pub fn parse_qname(res: &[u8]) -> String{
        let mut qname = String::from("");
        let mut pos: usize = 12; 

        loop {
            let size: u8 = res[pos]; 
            let my_vec: Vec<u8> = res[pos+1..pos+(size as usize)+1].to_vec();

            qname.push_str(&(String::from_utf8(my_vec)).unwrap());
            pos+=size as usize + 1;
            
            if res[pos] == 0 {
                break; 
            }
            else {
                qname.push_str(".");
            }
        }
        println!("{}", qname);
        return qname; 
    }

}