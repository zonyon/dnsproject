use crate::DnsQuestion::Dnsrtype::{A, AAAA, Cname, MX, NS, Ptr};

pub struct DnsQuestion {
    qname: String,
    Dnsrtype: Dnsrtype,
    qclass: u16,

}


pub enum Dnsrtype {
    A,
    AAAA,
    NS,
    MX,
    Cname,
    Ptr
}
impl Dnsrtype {
    pub fn no(&self) -> i32 {
        match self {
            Dnsrtype::A => return 1,
            Dnsrtype::AAAA => return 28,
            Dnsrtype::NS => return 2,
            Dnsrtype::MX => return 5,
            Dnsrtype::Cname => return 12,
            Dnsrtype::Ptr => return 15,
        }
    }
}
impl DnsQuestion {

    pub fn new(a: &str, b: Dnsrtype, mut c: u16) -> DnsQuestion {
        if (c != 0x0001) {
            eprintln!("Error: qclass not 0x0001");
            std::process::exit(1);
        }

        DnsQuestion {
            qname: a.to_string(),
            qclass: c,
            Dnsrtype: b ,

        }
    }


    pub fn qclass(&self) -> u16 {
        self.qclass
    }

    pub fn set_Dnsrtype(&mut self, Dnsrtype: Dnsrtype) {
        self.Dnsrtype = Dnsrtype;
    }
    pub fn set_qclass(&mut self, qclass: u16) {
        self.qclass = qclass;
    }
    pub fn Dnsrtype(&self) -> &Dnsrtype {
        &self.Dnsrtype
    }
    pub fn qname(&self) -> &str {
        &self.qname
    }
    pub fn set_qname(&mut self, qname: String) {
        self.qname = qname;
    }
}