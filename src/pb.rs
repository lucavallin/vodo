// BufferError is an enum that represents the various errors that can occur
#[derive(thiserror::Error, Debug, Clone)]
pub enum BufferError {
    #[error("End of buffer")]
    EndOfBuffer,
    #[error("Limit of {0} jumps exceeded")]
    JumpsLimitExceeded(i32),
    #[error("Single label exceeds 63 characters of length")]
    LabelTooLong,
    #[error("Generic error: {0}")]
    GenericError(String),
}

// Implement the From trait for BufferError, so that we can use the ? operator
impl From<std::io::Error> for BufferError {
    fn from(e: std::io::Error) -> Self {
        BufferError::GenericError(e.to_string())
    }
}

// The `PacketBuffer` struct is used to hold the contents of a DNS packet as a byte buffer,
// and provides methods for reading and manipulating the buffer contents.
pub struct PacketBuffer {
    pub buf: [u8; 512],
    pub pos: usize,
}

impl PacketBuffer {
    /// This gives us a fresh buffer for holding the packet contents, and a
    /// field for keeping track of where we are.
    pub fn new() -> PacketBuffer {
        PacketBuffer {
            buf: [0; 512],
            pos: 0,
        }
    }

    /// Current position within buffer
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Step the buffer position forward a specific number of steps
    pub fn step(&mut self, steps: usize) -> Result<(), BufferError> {
        self.pos += steps;

        Ok(())
    }

    /// Change the buffer position
    fn seek(&mut self, pos: usize) -> Result<(), BufferError> {
        self.pos = pos;

        Ok(())
    }

    /// Read a single byte and move the position one step forward
    fn read(&mut self) -> Result<u8, BufferError> {
        if self.pos >= 512 {
            return Err(BufferError::EndOfBuffer);
        }
        let res = self.buf[self.pos];
        self.pos += 1;

        Ok(res)
    }

    /// Get a single byte, without changing the buffer position
    fn get(&mut self, pos: usize) -> Result<u8, BufferError> {
        if pos >= 512 {
            return Err(BufferError::EndOfBuffer);
        }
        Ok(self.buf[pos])
    }

    /// Get a range of bytes
    pub fn get_range(&mut self, start: usize, len: usize) -> Result<&[u8], BufferError> {
        if start + len >= 512 {
            return Err(BufferError::EndOfBuffer);
        }
        Ok(&self.buf[start..start + len])
    }

    /// Read two bytes, stepping two steps forward
    pub fn read_u16(&mut self) -> Result<u16, BufferError> {
        let res = ((self.read()? as u16) << 8) | (self.read()? as u16);

        Ok(res)
    }

    /// Read four bytes, stepping four steps forward
    #[allow(clippy::identity_op)]
    pub fn read_u32(&mut self) -> Result<u32, BufferError> {
        let res = ((self.read()? as u32) << 24)
            | ((self.read()? as u32) << 16)
            | ((self.read()? as u32) << 8)
            | ((self.read()? as u32) << 0);

        Ok(res)
    }

    /// Read a qname
    ///
    /// The tricky part: Reading domain names, taking labels into consideration.
    /// Will take something like [3]www[6]google[3]com[0] and append
    /// www.google.com to outstr.
    pub fn read_qname(&mut self, outstr: &mut String) -> Result<(), BufferError> {
        // Since we might encounter jumps, we'll keep track of our position
        // locally as opposed to using the position within the struct. This
        // allows us to move the shared position to a point past our current
        // qname, while keeping track of our progress on the current qname
        // using this variable.
        let mut pos = self.pos();

        // track whether or not we've jumped
        let mut jumped = false;
        let max_jumps = 5;
        let mut jumps_performed = 0;

        // Our delimiter which we append for each label. Since we don't want a
        // dot at the beginning of the domain name we'll leave it empty for now
        // and set it to "." at the end of the first iteration.
        let mut delim = "";
        loop {
            // Dns Packets are untrusted data, so we need to be paranoid. Someone
            // can craft a packet with a cycle in the jump instructions. This guards
            // against such packets.
            if jumps_performed > max_jumps {
                return Err(BufferError::JumpsLimitExceeded(max_jumps));
            }

            // At this point, we're always at the beginning of a label. Recall
            // that labels start with a length byte.
            let len = self.get(pos)?;

            // If len has the two most significant bit are set, it represents a
            // jump to some other offset in the packet:
            if (len & 0xC0) == 0xC0 {
                // Update the buffer position to a point past the current
                // label. We don't need to touch it any further.
                if !jumped {
                    self.seek(pos + 2)?;
                }

                // Read another byte, calculate offset and perform the jump by
                // updating our local position variable
                let b2 = self.get(pos + 1)? as u16;
                let offset = (((len as u16) ^ 0xC0) << 8) | b2;
                pos = offset as usize;

                // Indicate that a jump was performed.
                jumped = true;
                jumps_performed += 1;

                continue;
            }
            // The base scenario, where we're reading a single label and
            // appending it to the output:
            else {
                // Move a single byte forward to move past the length byte.
                pos += 1;

                // Domain names are terminated by an empty label of length 0,
                // so if the length is zero we're done.
                if len == 0 {
                    break;
                }

                // Append the delimiter to our output buffer first.
                outstr.push_str(delim);

                // Extract the actual ASCII bytes for this label and append them
                // to the output buffer.
                let str_buffer = self.get_range(pos, len as usize)?;
                outstr.push_str(&String::from_utf8_lossy(str_buffer).to_lowercase());

                delim = ".";

                // Move forward the full length of the label.
                pos += len as usize;
            }
        }

        if !jumped {
            self.seek(pos)?;
        }

        Ok(())
    }

    // The write function writes a single byte to the buffer at the current position.
    // If the buffer is already full, it returns an EndOfBuffer error.
    pub fn write(&mut self, val: u8) -> Result<(), BufferError> {
        if self.pos >= 512 {
            return Err(BufferError::EndOfBuffer);
        }
        self.buf[self.pos] = val;
        self.pos += 1;
        Ok(())
    }

    // write_u8 writes a single byte to the buffer at the current position.
    pub fn write_u8(&mut self, val: u8) -> Result<(), BufferError> {
        self.write(val)?;

        Ok(())
    }

    // write_u16 writes two bytes to the buffer at the current position.
    // The most significant byte is written first.
    pub fn write_u16(&mut self, val: u16) -> Result<(), BufferError> {
        self.write((val >> 8) as u8)?;
        self.write((val & 0xFF) as u8)?;

        Ok(())
    }

    #[allow(clippy::identity_op)]
    // write_u32 writes four bytes to the buffer at the current position.
    // The most significant byte is written first.
    // This is useful for writing IPv4 addresses.
    pub fn write_u32(&mut self, val: u32) -> Result<(), BufferError> {
        self.write(((val >> 24) & 0xFF) as u8)?;
        self.write(((val >> 16) & 0xFF) as u8)?;
        self.write(((val >> 8) & 0xFF) as u8)?;
        self.write(((val >> 0) & 0xFF) as u8)?;

        Ok(())
    }

    // write_qname writes query names in labeled form
    pub fn write_qname(&mut self, qname: &str) -> Result<(), BufferError> {
        for label in qname.split('.') {
            // ox3f is 0011 1111 in binary, so we can use it to check if the label is longer than 63 characters
            let len = label.len();
            if len > 0x3f {
                return Err(BufferError::LabelTooLong);
            }

            self.write_u8(len as u8)?;
            for b in label.as_bytes() {
                self.write_u8(*b)?;
            }
        }

        self.write_u8(0)?;

        Ok(())
    }

    // set writes a single byte to the buffer at the specified position.
    fn set(&mut self, pos: usize, val: u8) -> Result<(), BufferError> {
        self.buf[pos] = val;

        Ok(())
    }

    // set_u16 writes two bytes to the buffer at the specified position.
    // The most significant byte is written first.
    pub fn set_u16(&mut self, pos: usize, val: u16) -> Result<(), BufferError> {
        self.set(pos, (val >> 8) as u8)?;
        // The bitwise AND operation with 0xFF then clears all bits except for the least significant byte,
        // which contains the value of the most significant byte of raw_addr.
        self.set(pos + 1, (val & 0xFF) as u8)?;

        Ok(())
    }
}
