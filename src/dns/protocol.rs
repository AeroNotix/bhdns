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
    fn read(buf: &mut Buffer) -> Result<Question, PackingError> {
        let mut join = "";
        let mut qname = String::new();
        loop {
            let label_size = buf.read_u8();
            if label_size == 0 {
                break;
            }
            println!("label size: {}", label_size);
            let label = buf.read_sized(label_size as usize);
            qname.push_str(join);
            qname.push_str(&String::from_utf8_lossy(label.as_slice()));
            join = ".";
        }
        let qtype = buf.read_u8();
        let qclass = buf.read_u16();
        Ok(Question {
            qname,
            qtype,
            qclass,
        })
    }
}

/*

https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.3

*/
#[derive(Debug)]
pub struct ResourceRecord {
    // TODO Parse these into the enum
    qname: String,
    address: u64,
}

impl ResourceRecord {
    fn read(buf: &mut Buffer) -> Result<ResourceRecord, PackingError> {
        let qname = buf.read_name();
        let qtype = buf.read_u16();
        let qclass = buf.read_u16();
        {
            // 32 bit ttl
            buf.read_u16();
            buf.read_u16();
        }
        let size = buf.read_u16();
        let rdata = buf.read_sized(size as usize);
        dbg!(qtype);
        dbg!(qclass);
        dbg!(rdata);
        Ok(ResourceRecord{qname: qname, address:112233})
    }
}

#[derive(Debug)]
pub struct Packet {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<ResourceRecord>,
    pub authority: Vec<ResourceRecord>,
    pub additional: Vec<ResourceRecord>,
}

impl Packet {
    fn new(header: Header) -> Packet {
        let questions = Vec::with_capacity(header.questions as usize);
        let answers = Vec::with_capacity(header.answers as usize);
        let authority = Vec::with_capacity(header.authoritative_entries as usize);
        let additional = Vec::with_capacity(header.additional_records as usize);
        Packet {
            header,
            answers,
            questions,
            authority,
            additional,
        }
    }
}

impl TryFrom<Vec<u8>> for Packet {
    type Error = PackingError;
    fn try_from(buf: Vec<u8>) -> Result<Self, Self::Error> {
        let header = Header::unpack_from_slice(&buf[0..12])?;
        let mut reader = Buffer::new(buf);

        // 13 because headers are 12 big
        let offset = 12;
        reader.seek(offset);

        let mut packet = Packet::new(header);

        for _ in 0..packet.header.questions {
            packet.questions.push(Question::read(&mut reader)?);
        }

        for _ in 0..packet.header.answers {
            packet.answers.push(ResourceRecord::read(&mut reader)?);
        }

        for _ in 0..packet.header.authoritative_entries {
            packet.authority.push(ResourceRecord::read(&mut reader)?);
        }

        for _ in 0..packet.header.additional_records {
            packet.additional.push(ResourceRecord::read(&mut reader)?);
        }

        Ok(packet)
    }
}

#[cfg(test)]
mod tests {}
