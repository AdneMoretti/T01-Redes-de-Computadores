use rand::Error;

pub struct DnsResponse {
    pub id: u16, 
    pub flags: u16, 
    pub qd_count: u16, 
    pub an_count: u16, 
    pub ns_count: u16, 
    pub ar_count: u16
}

impl DnsResponse {

    pub fn parse_header(&mut self, res: &[u8]){
        self.id = ((res[0] as u16) << 8) | (res[1] as u16);
        self.flags = ((res[2] as u16) << 8) + (res[3] as u16);
        self.qd_count = ((res[4] as u16) << 8) + (res[5] as u16);
        self.an_count = ((res[6] as u16) << 8) + (res[7] as u16);
        self.ns_count = ((res[8] as u16) << 8) | (res[9] as u16);
        self.ar_count = ((res[10] as u16) << 8) | (res[11] as u16);
    }

    pub fn get_response_code(&self, flag: u8, domain_name: &str) -> Result<(), &str> {
        let response_code: u8 = flag & 15; 
        println!("{}", response_code);
        match response_code {
            0=> return Ok(()),  
            1=> return Err("Format Error"), 
            2=> return Err("Server Failure"), 
            3=> return Err("Dominio nao encontrado"), 
            4=> return Err("Not Implemented"), 
            5=> return Err("Refused"), 
            _ => {
                return Err("Valor fora do intervalo esperado.");
            }
        }
    }

    pub fn parse_question(&self, res: &[u8]) -> (u16, u16){
        let q_type = ((res[0] as u16) << 8) + (res[1] as u16);
        let q_class = ((res[2] as u16) << 8) + (res[3] as u16);
        return (q_type, q_class);
    }

    pub fn parse_qname(&self, res: &[u8], byte: &mutusize) -> String{
        let mut qname = String::from("");
        let mut pos: usize = 0; 

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
            println!("{}", pos); 
        }
        return qname; 
    }

}