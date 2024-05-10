
pub mod types {
    pub struct DnsReponse {
        
    }
    pub struct DnsHeader {
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

}