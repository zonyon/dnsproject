use crate::DnsQuestion;

pub struct DnsRR {
    question : DnsQuestion::DnsQuestion,
    ttl: i32,
    rdlength: i16,
    rdata: u32
}
pub enum Dnsrtype {
    A,
    AAAA,
    NS,
    MX,
    Cname,
    Ptr
}
impl DnsRR {

    pub fn new(question_ : DnsQuestion::DnsQuestion) -> DnsRR {
        DnsRR {
            question : question_ ,
            ttl: 0,
            rdlength: 0,
            rdata: 0
        }
    }


    pub fn question(&self) -> &DnsQuestion::DnsQuestion {
        &self.question
    }
    pub fn ttl(&self) -> i32 {
        self.ttl
    }
    pub fn rdlength(&self) -> i16 {
        self.rdlength
    }
    pub fn rdata(&self) -> u32 {
        self.rdata
    }
    pub fn set_question(&mut self, question: DnsQuestion::DnsQuestion) {
        self.question = question;
    }
    pub fn set_ttl(&mut self, ttl: i32) {
        self.ttl = ttl;
    }
    pub fn set_rdlength(&mut self, rdlength: i16) {
        self.rdlength = rdlength;
    }
    pub fn set_rdata(&mut self, rdata: u32) {
        self.rdata = rdata;
    }
}