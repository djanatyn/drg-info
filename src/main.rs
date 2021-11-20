use std::io;
use std::env;
use std::fs;
use std::io::Read;
use deeprockgalactic_saveeditor::deep_rock_galactic::{SaveFile};

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("usage: {} <save-file>", &args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    let mut file = fs::File::open(filename)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let guids = fs::read_to_string("matrix_cores.json")?;

    let result = SaveFile::new(&mut data, &guids).expect("failed to parse");

    println!("{}", serde_json::to_string(&result.matrix_cores)?);

    Ok(())
}
