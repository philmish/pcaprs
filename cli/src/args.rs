use clap::Parser as clapParser;

#[derive(clapParser, Debug)]
#[clap(name = "pcaprs")]
#[clap(author = "philmish")]
#[clap(version = "0.1")]
pub struct Args {

    /// Pcap file to load
    #[clap(short, long)]
    file: String,

}

impl Args {

    pub fn f_name(&self) -> String {
        self.file.to_string()
    }

    pub fn init() -> Self {
        Self::parse()
    } 
}
