pub mod args;

use std::{fs::File, io::Read};

use args::Args;
use pcap::parser::PcapParser;

pub struct App {
    parser: PcapParser,
}

fn read_file(f_name: String) -> File {
    return File::open(f_name).unwrap();
}

impl App {

    pub fn from_file(f_name: String) -> Self {
        let mut data = read_file(f_name);
        let mut bytes = Vec::new();
        data.read_to_end(&mut bytes).unwrap();
        let parser = PcapParser::new(
            bytes.to_vec()
        );
        return Self{
            parser,
        };
    }

    pub fn show_header(&self) {
        self.parser.print_fh();
    }
}

pub fn run_app(args: Args) -> App {
    App::from_file(args.f_name())
}


