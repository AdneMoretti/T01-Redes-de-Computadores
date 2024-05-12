
pub struct DnsHeader {
    id: u16, 
    flags: u16, 
    qd_count: u16, 
    an_count: u16, 
    ar_count: u16    
}

pub mod parser_dns {
    use rand::Error;

    use super::DnsHeader;

    pub fn parse_response(res: &[u8]){
        //let header: DnsHeader::new();

        //let display_result = res
        //String::from_utf8(display_result).unwrap();
        //self::parse_id(res[0], res[1]); 
        let domain: String = self::parse_qname(res);
        // header: DnsHeader::new();

        //let display_result = res
        //String::from_utf8(display_result).unwrap();
        get_response_code(res[3], domain.as_str());
        
    }
    
    pub fn parse_id(id_1: u8, id_2: u8) -> u16{
        let record_type: u16 = ((id_1 as u16) << 8) | (id_2 as u16);
        return record_type; 
    }

    pub fn get_response_code(flag: u8, domain_name: &str) -> Result<(), &str> {
        let response_code: u8 = flag & 15; 
        println!("{}", response_code);
        match response_code {
            0=> return Ok(()),  
            1=> return Err("Format Error"), 
            2=> return Err("Server Failure"), 
            3=> return Err(format!("Dominio {} nao encontrado", domain_name).as_str()), 
            4=> return Err("Not Implemented"), 
            5=> return Err("Refused"), 
            _ => {
                return Err("Valor fora do intervalo esperado.");
            }
        }
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