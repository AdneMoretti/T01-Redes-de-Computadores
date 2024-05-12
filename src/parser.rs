pub struct DnsResponse {
    pub id: u16, 
    pub flags: u16, 
    pub qd_count: u16, 
    pub an_count: u16, 
    pub ns_count: u16, 
    pub ar_count: u16,

}

impl DnsResponse {

   pub fn new(res: &[u8]) -> Self {

        let mut dns_response: DnsResponse = DnsResponse {
            id: 0, 
            flags: 0, 
            qd_count: 0, 
            an_count: 0, 
            ns_count: 0, 
            ar_count: 0 
        };

        dns_response.parse_header(res);

        dns_response
    }


    pub fn parse_header(&mut self, res: &[u8]){
        self.id = ((res[0] as u16) << 8) | (res[1] as u16);
        self.flags = ((res[2] as u16) << 8) + (res[3] as u16);
        self.qd_count = ((res[4] as u16) << 8) + (res[5] as u16);
        self.an_count = ((res[6] as u16) << 8) + (res[7] as u16);
        self.ns_count = ((res[8] as u16) << 8) | (res[9] as u16);
        self.ar_count = ((res[10] as u16) << 8) | (res[11] as u16);
    }

    pub fn get_response_code(&self, flag: u8, domain_name: &str) -> Result<(), &'static str> {
        let response_code: u8 = flag & 15; 
        match response_code {
            0=> return Ok(()),  
            1=> return Err("Format Error"), 
            2=> return Err("Server Failure"), 
            3=> return Err(Box::leak(format!("Dominio {} nao encontrado", domain_name ).into_boxed_str())),
            4=> return Err("Not Implemented"), 
            5=> return Err("Refused"), 
            _ => {
                return Err("Valor fora do intervalo esperado.");
            }
        }
    }

    pub fn parse_question(&self, res: &[u8]) -> (u16, u16){
        let q_type = ((res[0] as u16) << 8) + (res[1] as u16);
        let q_class: u16 = ((res[2] as u16) << 8) + (res[3] as u16);
        
        

        return (q_type, q_class);
    }

    pub fn parse_qname(&self, res: &[u8], byte: &mut usize) -> String{
        let mut qname: String = String::from("");
        let mut pos: usize = 0; 

        loop {
            let size: u8 = res[pos]; 
            let my_vec: Vec<u8> = res[pos+1..pos+(size as usize)+1].to_vec();

            qname.push_str(&(String::from_utf8(my_vec)).unwrap());
            pos+=size as usize + 1;
            *byte += size as usize + 1;
            if res[pos] == 0 {
                break; 
            }
            
            else {
                qname.push_str(".");
            }
        }
        *byte += 1;
        return qname; 
    }

    pub fn parse_answer(&self, res: &[u8], byte: &mut usize) -> Option<String> {
        let q_type = ((res[2] as u16) << 8) + (res[3] as u16); 
        let mut rd_length = ((res[10] as u16) << 8) + (res[11] as u16); 
        let mut domain_name: String = String::from("");
        let mut pos: usize = 12; 

        if q_type != 2 {
            return None;
        }
        
        rd_length -= 2;

        let mut i: i32 = 0;
        *byte += 12;
        
        loop {
            let size: u8 = res[pos]; 
             let my_vec: Vec<u8> = res[pos+1..pos+(size as usize)+1].to_vec();

            domain_name.push_str(&(String::from_utf8(my_vec)).unwrap());
            pos+=size as usize + 1;
            i += (size as i32)+1;

            if i == rd_length.into() || res[pos] == 0 {
                break; 
            } else {
                domain_name.push_str(".");
            }
        }
        *byte += (rd_length as usize) + 2;
        return Some(domain_name);
    }

}
