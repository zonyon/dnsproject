use crate::{DnsHeader, DnsQuestion, Dnsrtype};
use crate::DnsRR::DnsRR;

pub struct DnsPacket {
    header: DnsHeader::DnsHeader,
    question: DnsQuestion::DnsQuestion,
    reponse: Vec<DnsRR>,
}

impl DnsPacket{

    pub fn genereRR(mut self){
        let answer: u16 = self.header.ancount();
        let authority: u16 = self.header.nscount();
        let additional: u16 = self.header.arcount();

        let rr = DnsRR::new(DnsQuestion::DnsQuestion::new(3 , DnsQuestion::Dnsrtype::A , 2 ));
        self.reponse.push(rr)


    }
}