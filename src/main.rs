use clap::Parser;
use deeprockgalactic_saveeditor::deep_rock_galactic::SaveFile;
use std::io::Read;
use std::{env, fs, io, path};

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "djanatyn")]
struct Opts {
    #[clap(parse(from_os_str))]
    save_file: path::PathBuf,
}

fn main() -> io::Result<()> {
    let opts = Opts::parse();

    let mut file = fs::File::open(&opts.save_file)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let guids = fs::read_to_string("matrix_cores.json")?;

    let result = SaveFile::new(&mut data, &guids).expect("failed to parse");

    println!("{}", serde_json::to_string(&result.matrix_cores)?);

    Ok(())
}
