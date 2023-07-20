use crate::bpb::{BytePacketBuffer, Result};
use crate::rc::ResultCode;

// This struct represents the DNS header and contains the following fields:
// - id: a 16-bit identifier assigned by the program that generates any kind of query or response
// - recursion_desired: a 1-bit field that specifies whether recursive query support is desired
// - truncated_message: a 1-bit field that specifies that this message was truncated due to length greater than that permitted on the transmission channel
// - authoritative_answer: a 1-bit field that specifies that the responding name server is an authority for the domain name in question section
// - opcode: a 4-bit field that specifies kind of query in this message
// - response: a 1-bit field that specifies whether this message is a response to a query or a query
// - rescode: a 4-bit field that specifies the response code
// - checking_disabled: a 1-bit field that specifies whether checking of query and response is disabled or not
// - authed_data: a 1-bit field that specifies whether all data in the response is authenticated
// - z: a 1-bit field that must be zero in all queries and responses
// - recursion_available: a 1-bit field that specifies whether recursive query support is available in the name server
// - questions: a 16-bit field that specifies the number of entries in the question section
// - answers: a 16-bit field that specifies the number of resource records in the answer section
// - authoritative_entries: a 16-bit field that specifies the number of name server resource records in the authority records section
// - resource_entries: a 16-bit field that specifies the number of resource records in the additional records section
#[derive(Clone, Debug)]
pub struct DnsHeader {
    pub id: u16, // 16 bits

    pub recursion_desired: bool,    // 1 bit
    pub truncated_message: bool,    // 1 bit
    pub authoritative_answer: bool, // 1 bit
    pub opcode: u8,                 // 4 bits
    pub response: bool,             // 1 bit

    pub rescode: ResultCode,       // 4 bits
    pub checking_disabled: bool,   // 1 bit
    pub authed_data: bool,         // 1 bit
    pub z: bool,                   // 1 bit
    pub recursion_available: bool, // 1 bit

    pub questions: u16,             // 16 bits
    pub answers: u16,               // 16 bits
    pub authoritative_entries: u16, // 16 bits
    pub resource_entries: u16,      // 16 bits
}

impl DnsHeader {
    pub fn new() -> DnsHeader {
        DnsHeader {
            id: 0,

            recursion_desired: false,
            truncated_message: false,
            authoritative_answer: false,
            opcode: 0,
            response: false,

            rescode: ResultCode::NOERROR,
            checking_disabled: false,
            authed_data: false,
            z: false,
            recursion_available: false,

            questions: 0,
            answers: 0,
            authoritative_entries: 0,
            resource_entries: 0,
        }
    }

    // This function reads the DNS header fields from a given BytePacketBuffer and updates the fields of the DnsHeader struct accordingly.
    // It reads the id, flags, rescode, questions, answers, authoritative_entries, and resource_entries fields from the buffer.
    // It then updates the corresponding fields in the DnsHeader struct with the values read from the buffer.
    // Finally, it returns a Result indicating whether the read was successful or not.
    pub fn read(&mut self, buffer: &mut BytePacketBuffer) -> Result<()> {
        self.id = buffer.read_u16()?;

        let flags = buffer.read_u16()?;
        let a = (flags >> 8) as u8;
        let b = (flags & 0xFF) as u8;
        self.recursion_desired = (a & (1 << 0)) > 0;
        self.truncated_message = (a & (1 << 1)) > 0;
        self.authoritative_answer = (a & (1 << 2)) > 0;
        self.opcode = (a >> 3) & 0x0F;
        self.response = (a & (1 << 7)) > 0;

        self.rescode = ResultCode::from_num(b & 0x0F);
        self.checking_disabled = (b & (1 << 4)) > 0;
        self.authed_data = (b & (1 << 5)) > 0;
        self.z = (b & (1 << 6)) > 0;
        self.recursion_available = (b & (1 << 7)) > 0;

        self.questions = buffer.read_u16()?;
        self.answers = buffer.read_u16()?;
        self.authoritative_entries = buffer.read_u16()?;
        self.resource_entries = buffer.read_u16()?;

        // Return the constant header size
        Ok(())
    }
}
