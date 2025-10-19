use fastwebsockets::Frame;

enum State {
    FinRsvOp,
    MaskLen,
    LenExt16(usize),
    LenExt64(usize),
    Mask(usize),
    Payload(u64),
}

pub struct FrameTransducer {
    fin: bool,
    opcode: fastwebsockets::OpCode,
    len: u64,
    mask: [u8; 4],
    payload: Vec<u8>,
    state: State,
}

impl FrameTransducer {
    pub fn new() -> Self {
        Self {
            fin: false,
            opcode: fastwebsockets::OpCode::Close,
            len: 0,
            mask: [0; 4],
            payload: Vec::new(),
            state: State::FinRsvOp,
        }
    }
    pub fn consume_bytes<'f>(
        &mut self,
        buf: &[u8],
    ) -> Result<Option<Frame<'f>>, fastwebsockets::WebSocketError> {
        for &byte in buf {
            if let Some(frame) = self.parse_next_byte(byte)? {
                return Ok(Some(frame));
            }
        }
        Ok(None)
    }

    fn parse_next_byte<'f>(
        &mut self,
        byte: u8,
    ) -> Result<Option<Frame<'f>>, fastwebsockets::WebSocketError> {
        match &mut self.state {
            State::FinRsvOp => {
                let (fin, opcode) = byte.into_fin_op()?;
                self.fin = fin;
                self.opcode = opcode;
                self.state = State::MaskLen;
            }
            State::MaskLen => {
                let len = byte.into_len()?;
                match len {
                    0 => {
                        self.len = 0;
                        let frame = fastwebsockets::Frame::new(
                            self.fin,
                            self.opcode,
                            None,
                            fastwebsockets::Payload::Owned(Vec::new()),
                        );
                        self.clean();
                        return Ok(Some(frame));
                    }
                    n @ 1..=125 => {
                        self.len = n as u64;
                        self.state = State::Mask(4);
                    }
                    126 => self.state = State::LenExt16(2),
                    127 => self.state = State::LenExt64(8),
                    _ => unreachable!(),
                };
            }
            State::LenExt16(leftover) => {
                self.len = (self.len << 8) | (byte as u64);

                let next = *leftover - 1;
                if next == 0 {
                    self.state = State::Mask(4);
                } else {
                    self.state = State::LenExt16(next);
                }
            }
            State::LenExt64(leftover) => {
                self.len = (self.len << 8) | (byte as u64);

                let next = *leftover - 1;
                if next == 0 {
                    self.state = State::Mask(4);
                } else {
                    self.state = State::LenExt64(next);
                }
            }
            State::Mask(leftover) => {
                self.mask[4 - *leftover] = byte;

                let next = *leftover - 1;
                if next == 0 {
                    self.state = State::Payload(self.len);
                } else {
                    self.state = State::Mask(next);
                }
            }
            State::Payload(leftover) => {
                let next = *leftover - 1;
                self.payload.push(byte);
                if next == 0 {
                    let frame = fastwebsockets::Frame::new(
                        self.fin,
                        self.opcode,
                        Some(self.mask),
                        fastwebsockets::Payload::Owned(self.payload.clone()),
                    );

                    self.clean();
                    return Ok(Some(frame));
                } else {
                    self.state = State::Payload(next);
                }
            }
        }
        Ok(None)
    }

    fn clean(&mut self) -> () {
        self.fin = false;
        self.opcode = fastwebsockets::OpCode::Close;
        self.len = 0;
        self.mask = [0; 4];
        self.payload = Vec::new();
        self.state = State::FinRsvOp;
    }
}

trait FrameByteExt {
    fn into_fin_op(self) -> Result<(bool, fastwebsockets::OpCode), fastwebsockets::WebSocketError>;
    fn into_len(self) -> Result<u8, fastwebsockets::WebSocketError>;
}
impl FrameByteExt for u8 {
    fn into_fin_op(self) -> Result<(bool, fastwebsockets::OpCode), fastwebsockets::WebSocketError> {
        if self & 0b0111_0000 != 0 {
            return Err(fastwebsockets::WebSocketError::ReservedBitsNotZero);
        }
        let fin = (self & 0b1000_0000) != 0;
        let opcode = self & 0b0000_1111;
        Ok((fin, opcode.try_into()?))
    }

    fn into_len(self) -> Result<u8, fastwebsockets::WebSocketError> {
        let masked = (self & 0b1000_0000) != 0;
        if !masked {
            return Err(fastwebsockets::WebSocketError::InvalidValue);
        }
        Ok(self & 0b0111_1111)
    }
}
