use std::convert::{Into, TryFrom};
use packed_struct::prelude::*;

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

DNS header packet layout

+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
| 0| 1| 2| 3| 4| 5| 6| 7| 0| 1| 2| 3| 4| 5| 6| 7|
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|                      ID                       |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|QR| Opcode |AA|TC|RD|RA|  Z  |      RCODE      |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|                    QDCOUNT                    |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|                    ANCOUNT                    |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|                    NSCOUNT                    |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|                    ARCOUNT                    |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+

ID: any arbitrary 16 bit identifier, the ID should be passed back in
responses to queries.

QR: 0 = query, 1 = response
Opcode: 4 bits representing the type of response. See ResponseCode.
AA: authoritative answer
TC: is message truncated?
RD: is recursion desired?
RA: is recursion available?
Z:  unused / future use. I think this can get used for secure DNS?
QDCOUNT: count of questions
ANCOUNT: count of answers
ARCOUNT: count of additional records
*/

#[derive(Debug, PackedStruct)]
#[packed_struct(bit_numbering="msb0", endian="msb")]
pub struct Header {
    #[packed_field(bits="0..16")]
    pub id: u16,

    #[packed_field(size_bits="1")]
    pub is_response: bool,

    #[packed_field(size_bits="4")]
    pub opcode: u8,

    #[packed_field(size_bits="1")]
    pub is_authoritative_answer: bool,

    #[packed_field(size_bits="1")]
    pub is_truncated_message: bool,

    #[packed_field(size_bits="1")]
    pub is_recursion_desired: bool,

    #[packed_field(size_bits="1")]
    pub is_recursion_available: bool,

    #[packed_field(size_bits="3")]
    pub z: u8,

    #[packed_field(size_bits="4", ty="enum")]
    pub response_code: ResponseCode,

    #[packed_field(size_bits="16")]
    pub questions: u16,

    #[packed_field(size_bits="16")]
    pub answers: u16,

    #[packed_field(size_bits="16")]
    pub authoritative_entries: u16,

    #[packed_field(size_bits="16")]
    pub additional_records: u16,
}

impl TryFrom<&[u8]> for Header {
    type Error = PackingError;
    fn try_from(item: &[u8]) -> Result<Self, Self::Error> {
        Header::unpack_from_slice(item)
    }
}

#[cfg(test)]
mod tests {

}
