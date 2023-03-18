use crate::record::{Record, RecordHeader};

enum ParserState {
    Header,
    Body
}

pub struct RecordParser {
    state: ParserState,
    active_header: [u8;16],
    active_data: Vec<u8>,
    curr_offset: usize,
    curr_pos: usize,
    curr_body_len: usize,
    packets: Vec<Record>,
    is_swapped: bool,
}


impl RecordParser {

    pub fn new(swapped: bool) -> Self {
        RecordParser{
            state: ParserState::Header,
            active_header: [0;16],
            active_data: vec![],
            curr_offset: 0,
            curr_pos: 0,
            curr_body_len: 0,
            packets: vec![],
            is_swapped: swapped,
        }
    }

    fn header_check(&mut self) {
        if self.active_header.len() == 16 && self.curr_pos == 16 {
            self.state = ParserState::Body;
            self.curr_pos = 0;
            let tmph = RecordHeader::new(self.active_header, self.is_swapped);
            self.curr_body_len = tmph.cap_len() as usize;
        }
    }

    fn body_check(&mut self) {
        if  self.curr_pos == self.curr_body_len {
            self.packets.push(
                Record::new(RecordHeader::new(
                        self.active_header, self.is_swapped),
                        self.active_data.to_vec()
                )
            );
            self.state = ParserState::Header;
            self.active_header = [0;16];
            self.active_data = vec![];
            self.curr_pos = 0;
        }
    }

    fn put_byte(&mut self, byte: u8) {
        match self.state {
            ParserState::Header => self.active_header[self.curr_pos] = byte,
            ParserState::Body => self.active_data.push(byte),
        }
        self.curr_pos += 1;
        self.curr_offset += 1;
    }

    fn check_switch(&mut self) {
        match self.state {
            ParserState::Header => self.header_check(),
            ParserState::Body => self.body_check(),
        }
    }

    pub fn parse_records(&mut self, data: Vec<u8>, offset: usize) -> Vec<Record> {
        let bytes = get_vec_from_offset(data, offset);
        for byte in bytes.into_iter() {
            self.check_switch();
            self.put_byte(byte);
        }
        self.packets.to_vec()
    }
}

fn get_vec_from_offset(v: Vec<u8>, n: usize) -> Vec<u8> {
    let mut c: usize = 0;
    let mut res: Vec<u8> = vec![];
    let _: Vec<()> =  v.into_iter().map(|x| {
        if c >= n {
            res.push(x)
        }
        c += 1;
    }).collect();
    res
}
