use crate::DnsQuestion::Dnsrtype;

mod DnsHeader;
mod DnsPacket;
mod DnsQuestion;
mod DnsRR;



fn main() {

    let aa = DnsQuestion::DnsQuestion::new("test",DnsQuestion::Dnsrtype::AAAA,0x0001);

    let b = DnsHeader::DnsHeader::new(true,true,true,true,5,3,4,7,5) ;
    let mut listRR: Vec<DnsRR::DnsRR> = vec![];

    let c = DnsRR::DnsRR::new(DnsQuestion::DnsQuestion::new("test2", Dnsrtype::AAAA, 0x0001), ) ;
    unsafe { listRR.push(c); }
    let k = DnsRR::DnsRR::new(DnsQuestion::DnsQuestion::new("test", Dnsrtype::AAAA, 0x0001), ) ;
    unsafe { listRR.push(k); }

    let d = DnsPacket::DnsPacket::new(b, aa,  listRR ) ;
    println!("{}", d.byte_size());
}
