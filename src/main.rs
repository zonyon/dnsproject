use crate::DnsRR::Dnsrtype;

mod DnsPacket{
    use crate::{DnsHeader, DnsQuestion};
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



    }
}

}




mod DnsHeader {
    use std::io::Write;
    use rand::Rng;
    static mut LIST_ID: Vec<u16> = vec![];
    pub struct DnsHeader {
        id: u16,
        qr: bool,
        opcode: bool,
        aa: bool,
        tc: bool,
        rd: bool,
        ra: bool,
        z: bool,
        rcode: i16,
        qdcount: u16,
        ancount: u16,
        nscount: u16,
        arcount: u16,

    }
    impl DnsHeader {

        pub(crate) fn new(a: bool, b: bool, c: bool, d: bool, e: u16, f: u16, g: u16, h: i16 ,  i: u16) -> Self {
            //attribution d'un id unique et aléatoire
            let mut temp: u16 = rand::thread_rng().gen();
            unsafe {
                while LIST_ID.contains(&temp) {
                    temp = rand::thread_rng().gen();
                }
            }
            DnsHeader {
                id: temp,
                qr: a,
                opcode: false,
                aa: b,
                tc: c,
                rd: true,
                ra: d,
                z: false,
                rcode: h,
                qdcount: e,
                ancount: f,
                nscount: g,
                arcount: i
            }
        }
        pub fn id(&self) -> u16 {
            self.id
        }
        pub fn qr(&self) -> bool {
            self.qr
        }
        pub fn opcode(&self) -> bool {
            self.opcode
        }
        pub fn aa(&self) -> bool {
            self.aa
        }
        pub fn tc(&self) -> bool {
            self.tc
        }
        pub fn rd(&self) -> bool {
            self.rd
        }
        pub fn ra(&self) -> bool {
            self.ra
        }
        pub fn z(&self) -> bool {
            self.z
        }
        pub fn rcode(&self) -> i16 {
            self.rcode
        }
        pub fn qdcount(&self) -> u16 {
            self.qdcount
        }
        pub fn ancount(&self) -> u16 {
            self.ancount
        }
        pub fn nscount(&self) -> u16 {
            self.nscount
        }
        pub fn arcount(&self) -> u16 {
            self.arcount
        }
        pub fn set_id(&mut self, id: u16) {
            self.id = id;
        }
        pub fn set_qr(&mut self, qr: bool) {
            self.qr = qr;
        }
        pub fn set_opcode(&mut self, opcode: bool) {
            self.opcode = opcode;
        }
        pub fn set_aa(&mut self, aa: bool) {
            self.aa = aa;
        }
        pub fn set_tc(&mut self, tc: bool) {
            self.tc = tc;
        }
        pub fn set_rd(&mut self, rd: bool) {
            self.rd = rd;
        }
        pub fn set_ra(&mut self, ra: bool) {
            self.ra = ra;
        }
        pub fn set_z(&mut self, z: bool) {
            self.z = z;
        }
        pub fn set_rcode(&mut self, rcode: i16) {
            self.rcode = rcode;
        }
        pub fn set_qdcount(&mut self, qdcount: u16) {
            self.qdcount = qdcount;
        }
        pub fn set_ancount(&mut self, ancount: u16) {
            self.ancount = ancount;
        }
        pub fn set_nscount(&mut self, nscount: u16) {
            self.nscount = nscount;
        }
        pub fn set_arcount(&mut self, arcount: u16) {
            self.arcount = arcount;
        }
    }
}
mod DnsQuestion {
    use crate::DnsQuestion::Dnsrtype::{A, AAAA, Cname, MX, NS, Ptr};

    pub struct DnsQuestion {
        qname: u16,
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

        pub fn new(a: u16, b: Dnsrtype, mut c: u16) -> DnsQuestion {
            if (c != 0x0001) {
                eprintln!("Error: qclass not 0x0001");
                std::process::exit(1);
            }

            DnsQuestion {
                qname: a,
                qclass: c,
                Dnsrtype: b
            }
        }

        pub fn qname(&self) -> u16 {
            self.qname
        }
        pub fn qclass(&self) -> u16 {
            self.qclass
        }
        pub fn set_qname(&mut self, qname: u16) {
            self.qname = qname;
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
    }
}

mod DnsRR {
    use crate::DnsQuestion;
    pub struct DnsRR {
        rname: u16,
        Dnsrtype: Dnsrtype,
        rclass: u16,
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

        pub fn new(a: u16, b: Dnsrtype, c: u16) -> DnsRR {
            if c != 0x0001 {
                eprintln!("Error: qclass not 0x0001");
                std::process::exit(1);
            }
            DnsRR {
                rname: a,
                Dnsrtype: b,
                rclass: c,
                ttl: 0,
                rdlength: 0,
                rdata: 0
            }
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

        pub fn set_ttl(&mut self, ttl: i32) {
            self.ttl = ttl;
        }
        pub fn set_rdlength(&mut self, rdlength: i16) {
            self.rdlength = rdlength;
        }
        pub fn set_rdata(&mut self, rdata: u32) {
            self.rdata = rdata;
        }
        pub fn rname(&self) -> u16 {
            self.rname
        }

        pub fn rclass(&self) -> u16 {
            self.rclass
        }
        pub fn set_rname(&mut self, rname: u16) {
            self.rname = rname;
        }
        pub fn set_Dnsrtype(&mut self, Dnsrtype: Dnsrtype) {
            self.Dnsrtype = Dnsrtype;
        }
        pub fn set_rclass(&mut self, rclass: u16) {
            self.rclass = rclass;
        }
        pub fn Dnsrtype(&self) -> &Dnsrtype {
            &self.Dnsrtype
        }
    }
}

fn main() {
    let a = DnsQuestion::Dnsrtype::AAAA;
    println!("{}",a.no());


}
