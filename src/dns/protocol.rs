use crate::dns::buffer::Buffer;
use packed_struct::prelude::*;
use std::convert::{Into, TryFrom};

#[allow(clippy::upper_case_acronyms)]
#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum ResponseCode {
    NOERROR = 0,
    FORMERR = 1,
    SERVFAIL = 2,
    NXDOMAIN = 3,
    NOTIMP = 4,
    REFUSED = 5,
}

/*
From: https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.1
*/

#[derive(Debug, PackedStruct)]
#[packed_struct(bit_numbering = "msb0", endian = "msb")]
pub struct Header {
    #[packed_field(bits = "0..16")]
    pub id: u16,

    #[packed_field(size_bits = "1")]
    pub is_response: bool,

    #[packed_field(size_bits = "4")]
    pub opcode: u8,

    #[packed_field(size_bits = "1")]
    pub is_authoritative_answer: bool,

    #[packed_field(size_bits = "1")]
    pub is_truncated_message: bool,

    #[packed_field(size_bits = "1")]
    pub is_recursion_desired: bool,

    #[packed_field(size_bits = "1")]
    pub is_recursion_available: bool,

    #[packed_field(size_bits = "3")]
    pub z: u8,

    #[packed_field(size_bits = "4", ty = "enum")]
    pub response_code: ResponseCode,

    #[packed_field(size_bits = "16")]
    pub questions: u16,

    #[packed_field(size_bits = "16")]
    pub answers: u16,

    #[packed_field(size_bits = "16")]
    pub authoritative_entries: u16,

    #[packed_field(size_bits = "16")]
    pub additional_records: u16,
}

impl TryFrom<&[u8]> for Header {
    type Error = PackingError;
    fn try_from(item: &[u8]) -> Result<Self, Self::Error> {
        Header::unpack_from_slice(item)
    }
}

#[allow(clippy::upper_case_acronyms)]
pub enum QueryType {
    // Source: https://en.wikipedia.org/wiki/List_of_DNS_record_types
    A = 1,
    AAAA = 28,
    AFSDB = 18,
    APL = 42,
    AXFR = 252,
    CAA = 257,
    CDNSKEY = 60,
    CDS = 59,
    CERT = 37,
    CNAME = 5,
    CSYNC = 62,
    DHCID = 49,
    DLV = 32769,
    DNAME = 39,
    DNSKEY = 48,
    DS = 43,
    EUI48 = 108,
    EUI64 = 109,
    HINFO = 13,
    HIP = 55,
    HTTPS = 65,
    IPSECKEY = 45,
    IXFR = 251,
    KEY = 25,
    KX = 36,
    LOC = 29,
    MX = 15,
    NAPTR = 35,
    NS = 2,
    NSEC = 47,
    NSEC3 = 50,
    NSEC3PARAM = 51,
    OPENPGPKEY = 61,
    OPT = 41,
    PTR = 12,
    RP = 17,
    RRSIG = 46,
    SIG = 24,
    SMIMEA = 53,
    SOA = 6,
    SRV = 33,
    SSHFP = 44,
    SVCB = 64,
    TA = 32768,
    TKEY = 249,
    TLSA = 52,
    TSIG = 250,
    TXT = 16,
    URI = 256,
    ZONEMD = 63,
}

#[allow(clippy::upper_case_acronyms)]
pub enum QueryClass {
    INTERNET = 0x0001,
    CSNET = 0x0002,
    CHAOS = 0x0003,
    HESIOD = 0x0004,
    NONE = 0x00fe,
    ALLANY = 0x00ff,
}

#[derive(Debug)]
pub struct Question {
    // TODO Parse these into the enum
    qname: String,
    qtype: u8,   //QueryType,
    qclass: u16, //QueryClass,
}

impl Question {
    fn parse(count: u16, buf: Vec<u8>) -> Result<Vec<Question>, ()> {
        let mut questions = Vec::with_capacity(count as usize);
        let mut b = Buffer::new(buf);
        // 13 because headers are 12 big
        let offset = 12;
        b.seek(offset);

        println!("{}", count);
        // TODO handle jumps
        for _ in 0..count {
            let mut join = "";
            let mut qname = String::new();
            loop {
                let label_size = b.read_u8();
                if label_size == 0 {
                    break;
                }
                println!("label size: {}", label_size);
                let label = b.read_sized(label_size as usize);
                qname.push_str(join);
                qname.push_str(&String::from_utf8_lossy(label));
                join = ".";
            }
            let qtype = b.read_u8();
            let qclass = b.read_u16();
            questions.push(Question {
                qname,
                qtype,
                qclass,
            });
        }

        Ok(questions)
    }
}

#[derive(Debug)]
pub struct Answer {
    // TODO Parse these into the enum
    qname: String,
    address: u64,
}

impl Answer {
    fn parse(count: u16, buf: Vec<u8>) -> Result<Vec<Answer>, ()> {
        return Ok(Vec::new());
    }
}

pub struct Packet {
    pub header: Header,
    pub questions: Vec<Question>,
    // pub answers: Vec<Answer>,
}

impl TryFrom<Vec<u8>> for Packet {
    type Error = PackingError;
    fn try_from(buf: Vec<u8>) -> Result<Self, Self::Error> {
        let header = Header::unpack_from_slice(&buf[0..12])?;
        let questions = Question::parse(header.questions, buf).unwrap();
        // let answers = Answer::parse(header.answers, buf).unwrap();
        Ok(Packet {
            header,
            questions,
            // answers,
        })
    }
}

#[cfg(test)]
mod tests {}
