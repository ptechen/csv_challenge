use structopt_derive::*;
// use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "csv_challenge", about = "Usage")]
pub struct Opt {
    #[structopt(help = "Input file")]
    pub input: String,
    #[structopt(help = "Column Name")]
    pub column_name: String,
    #[structopt(help = "Replacement Column Name")]
    pub replacement: String,
    #[structopt(help = "Output file, stdout if not present")]
    pub output: Option<String>,
}