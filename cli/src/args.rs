use clap::Parser as clapParser;

#[derive(clapParser, Debug)]
#[clap(name = "pcaprs")]
#[clap(author = "philmish")]
#[clap(version = "0.1")]
pub struct Args {

    /// Pcap file to load
    #[clap(short, long)]
    file: String,

    /// Record index to print
    #[clap(short, long)]
    idx: usize,

}

impl Args {

    pub fn f_name(&self) -> String {
        self.file.to_string()
    }

    pub fn index(&self) -> usize {
        self.idx
    }

    pub fn init() -> Self {
        Self::parse()
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args() {
        let args = Args{file: "test.file".to_string(), idx: 2, };
        assert_eq!(args.f_name(), "test.file".to_string());
        assert_eq!(args.index(), 2);
    }
}
