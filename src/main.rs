use std::path::PathBuf;
use structopt::StructOpt;
use rocco::{Docco};
use anyhow::Error;

#[derive(Debug, StructOpt)]
#[structopt(bin_name = "cargo")]
pub enum Command {
    #[structopt(name = "docco")]
    Docco(Opt)
}

#[derive(Debug, StructOpt)]
#[structopt(name = "docco", about = "Literate-style documentation generator from source code")]
pub struct Opt {
    #[structopt(parse(from_os_str), short="i", help="input source file")]
    input: PathBuf,
    #[structopt(parse(from_os_str), short="o", help="optional path to the generated output html file")]
    output: Option<PathBuf>
}

fn main() -> Result<(), Error> {
    let opt = Command::from_args();
    match opt {
        Command::Docco(opt) => {
            let mut docco = Docco::new(opt.input, opt.output)?;
            docco.parse()?;
            docco.render()?;
        }
    }
    Ok(())
}
