pub mod args;

use std::{fs::File, io::Read};

use args::Args;
use pcap::parser::RecordParser;
use pcap::file::FileHeader;
use pcap::record::Record;

pub struct App {
    header: FileHeader,
    records: Vec<Record>,
}

fn read_file(f_name: String) -> File {
    return File::open(f_name).unwrap();
}

impl App {

    pub fn from_file(f_name: String) -> Self {
        let mut data = read_file(f_name);
        let mut bytes = Vec::new();
        data.read_to_end(&mut bytes).unwrap();
        let file_h = FileHeader::new(bytes.to_vec()).unwrap();
        let mut parser = RecordParser::new(file_h.is_swapped());
        let records = parser.parse_records(bytes.to_vec(), 24);
        return Self{
            header: file_h,
            records
        };
    }

    pub fn show_header(&mut self) {
        println!("{}", self.header);
    }

    pub fn print_packet(&self, n: usize) {
        if n + 1  > self.records.len() {
            println!("Index out of bounds")
        } else {
            println!("{}", self.records[n]);
        }
    }
}

pub fn run_app(args: Args) -> App {
    App::from_file(args.f_name())
}
