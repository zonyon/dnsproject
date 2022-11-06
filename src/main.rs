mod DnsHeader;
mod DnsPacket;
mod DnsQuestion;
mod DnsRR;

use crate::DnsRR::Dnsrtype;


fn main() {
    let a = DnsQuestion::Dnsrtype::AAAA;
    println!("{}",a.no());


}
