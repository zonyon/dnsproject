use crate::{DnsHeader, DnsQuestion};
use crate::DnsRR::DnsRR;

pub struct DnsPacket {
    header: DnsHeader::DnsHeader,
    question: DnsQuestion::DnsQuestion,
    pub reponse: Vec<DnsRR>,
}

impl DnsPacket{


    pub fn byte_size(&self)-> i32{
        let a = 18 + (self.reponse.len()*12) ;
        return a.try_into().unwrap() ;
    }


    pub fn new(a : DnsHeader::DnsHeader, b : DnsQuestion::DnsQuestion, c : Vec<DnsRR>) -> DnsPacket {
        DnsPacket {
            header: a ,
            question: b ,
            reponse: c
        }
    }

    pub fn genereRR(mut self){
        let answer: u16 = self.header.ancount();
        let authority: u16 = self.header.nscount();
        let additional: u16 = self.header.arcount();


    }


}