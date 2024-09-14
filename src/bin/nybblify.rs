// Convert binary file to a nybbled format.
// Replace every byte with two bytes, which are the nybbles of the original byte.

use std::path::{Path, PathBuf};
use std::fs;
use std::io::{Read, Write};
use nybble::{NybbleOrder, nybblify};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    infile: PathBuf,

    #[arg(short, long)]
    outfile: PathBuf,

    #[arg(short = 'n', long = "nybble-order")]
    order: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let mut order = NybbleOrder::HighFirst;

    if let Some(order_string) = cli.order {
        if order_string.starts_with('l') {
            order = NybbleOrder::LowFirst;
        }
        else if order_string.starts_with('h') {
            order = NybbleOrder::HighFirst;
        }
        else {
            eprintln!("Invalid nybble order: {}", order_string);
            std::process::exit(1);
        }
    }

    if let Some(buffer) = read_file(&cli.infile) {
        let data = nybblify(buffer, order);
        let path = Path::new(&cli.outfile);
        let display = path.display();
        let mut file = match fs::File::create(path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(&data) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => { },
        }
    }
}

pub fn read_file(name: &Path) -> Option<Vec<u8>> {
    match fs::File::open(&name) {
        Ok(mut f) => {
            let mut buffer = Vec::new();
            match f.read_to_end(&mut buffer) {
                Ok(_) => Some(buffer),
                Err(_) => None
            }
        },
        Err(_) => {
            eprintln!("Unable to open file {}", &name.display());
            None
        }
    }
}
