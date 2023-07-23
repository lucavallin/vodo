use crate::pb::{BufferError, PacketBuffer};
use crate::rc::ResultCode;

/// This struct represents the DNS header and contains the following fields:
/// - id: a 16-bit identifier assigned by the program that generates any kind of query or response
/// - recursion_desired: a 1-bit field that specifies whether recursive query support is desired
/// - truncated_message: a 1-bit field that specifies that this message was truncated due to length greater than that permitted on the transmission channel
/// - authoritative_answer: a 1-bit field that specifies that the responding name server is an authority for the domain name in question section
/// - opcode: a 4-bit field that specifies kind of query in this message
/// - response: a 1-bit field that specifies whether this message is a response to a query or a query
/// - rescode: a 4-bit field that specifies the response code
/// - checking_disabled: a 1-bit field that specifies whether checking of query and response is disabled or not
/// - authed_data: a 1-bit field that specifies whether all data in the response is authenticated
/// - z: a 1-bit field that must be zero in all queries and responses
/// - recursion_available: a 1-bit field that specifies whether recursive query support is available in the name server
/// - questions: a 16-bit field that specifies the number of entries in the question section
/// - answers: a 16-bit field that specifies the number of resource records in the answer section
/// - authoritative_entries: a 16-bit field that specifies the number of name server resource records in the authority records section
/// - resource_entries: a 16-bit field that specifies the number of resource records in the additional records section
#[derive(Clone, Debug)]
pub struct DnsHeader {
    pub id: u16,

    pub recursion_desired: bool,
    pub truncated_message: bool,
    pub authoritative_answer: bool,
    pub opcode: u8,
    pub response: bool,

    pub rescode: ResultCode,
    pub checking_disabled: bool,
    pub authed_data: bool,
    pub z: bool,
    pub recursion_available: bool,

    pub questions: u16,
    pub answers: u16,
    pub authoritative_entries: u16,
    pub resource_entries: u16,
}

impl DnsHeader {
    pub fn new() -> DnsHeader {
        DnsHeader {
            // id is usually set to a random number by the client
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

    /// This function reads the DNS header fields from a given PacketBuffer and updates the fields of the DnsHeader struct accordingly.
    /// It reads the id, flags, rescode, questions, answers, authoritative_entries, and resource_entries fields from the buffer.
    /// It then updates the corresponding fields in the DnsHeader struct with the values read from the buffer.
    /// Finally, it returns a Result indicating whether the read was successful or not.
    /// Notice: Bits are shifted by (position of the fields in the header + size of the field)
    pub fn read(&mut self, buffer: &mut PacketBuffer) -> Result<(), BufferError> {
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

    /// This function writes the DNS header fields to a given PacketBuffer.
    /// It writes the id, flags, rescode, questions, answers, authoritative_entries, and resource_entries fields to the buffer.
    /// Finally, it returns a Result indicating whether the write was successful or not.
    /// Notice: Bits are shifted by (position of the fields in the header + size of the field)
    pub fn write(&self, buffer: &mut PacketBuffer) -> Result<(), BufferError> {
        buffer.write_u16(self.id)?;

        buffer.write_u8(
            (self.recursion_desired as u8)
                | ((self.truncated_message as u8) << 1)
                | ((self.authoritative_answer as u8) << 2)
                | (self.opcode << 3)
                | ((self.response as u8) << 7),
        )?;

        buffer.write_u8(
            (self.rescode as u8)
                | ((self.checking_disabled as u8) << 4)
                | ((self.authed_data as u8) << 5)
                | ((self.z as u8) << 6)
                | ((self.recursion_available as u8) << 7),
        )?;

        buffer.write_u16(self.questions)?;
        buffer.write_u16(self.answers)?;
        buffer.write_u16(self.authoritative_entries)?;
        buffer.write_u16(self.resource_entries)?;

        Ok(())
    }
}
