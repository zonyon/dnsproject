mod DnsHeader;
mod DnsPacket;
mod DnsQuestion;
mod DnsRR;



fn main() {
    let a = DnsQuestion::Dnsrtype::AAAA;
    println!("{}",a.no());
    let b = DnsHeader::DnsHeader::new(true,true,true,true,5,3,4,7,5) ;
    static mut listRR: Vec<DnsRR::DnsRR> = vec![];
    // let c = DnsRR::DnsRR::new(DnsHeader::DnsHeader::new(true,true,true,true,5,3,4,4,4)) ;


}
