use clap::Parser;
use deeprockgalactic_saveeditor::deep_rock_galactic::{
    Cosmetic, MatrixCores, Overclock, OverclockState, SaveFile,
};
use serde::Serialize;
use std::io::Read;
use std::{env, fs, io, path};

/// Command line interface to localcc/deeprockgalactic-saveeditor features.
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "djanatyn <djanatyn@gmail.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    /// Parse save file and output as JSON.
    Parse(Parse),
    /// Output missing overclocks and cosmetics from save file as JSON.
    Missing(Missing),
}

#[derive(Parser, Debug)]
struct Parse {
    /// Path to save file.
    #[clap(parse(from_os_str))]
    save_file: path::PathBuf,
}

#[derive(Parser, Debug)]
struct Missing {
    /// Path to save file.
    #[clap(parse(from_os_str))]
    save_file: path::PathBuf,
}

#[derive(Debug, Serialize)]
struct MissingCores {
    overclocks: Vec<Overclock>,
    cosmetics: Vec<Cosmetic>,
}

fn missing_cores(cores: MatrixCores) -> MissingCores {
    let missing_overclocks = cores
        .overclocks
        .values()
        .filter(|overclock| overclock.state == OverclockState::Unacquired)
        .map(|overclock| overclock.clone())
        .collect::<Vec<_>>();

    let missing_cosmetics = cores
        .cosmetics
        .values()
        .filter(|cosmetic| cosmetic.state == OverclockState::Unacquired)
        .map(|cosmetic| cosmetic.clone())
        .collect::<Vec<_>>();

    MissingCores {
        overclocks: missing_overclocks,
        cosmetics: missing_cosmetics,
    }
}

fn load_file(path: &path::PathBuf) -> io::Result<SaveFile> {
    let mut file = fs::File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let guids = fs::read_to_string("matrix_cores.json")?;

    Ok(SaveFile::new(&mut data, &guids).expect("failed to parse"))
}

fn main() -> io::Result<()> {
    let opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Parse(args) => {
            let save = load_file(&args.save_file)?;
            println!("{}", serde_json::to_string(&save.matrix_cores)?);
        }
        SubCommand::Missing(args) => {
            let save = load_file(&args.save_file)?;
            let missing = missing_cores(save.matrix_cores);
            println!("{}", serde_json::to_string(&missing)?);
        }
    }

    Ok(())
}
