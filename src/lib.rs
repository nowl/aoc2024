use std::{
    fs::File,
    io::{self, BufRead, Read},
    path::{Path, PathBuf},
};

use clap::Parser;

pub fn read_as_lines(path: &Path) -> io::Result<Vec<String>> {
    let mut file = File::open(path)?;

    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let mut lines = buf.split("\n").map(|s| s.to_owned()).collect::<Vec<_>>();

    if lines.last().map_or(false, |v| v == "") {
        lines.remove(lines.len() - 1);
    }

    Ok(lines)
}

pub fn read_line() -> io::Result<()> {
    // read line
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_line(&mut buffer)?;

    Ok(())
}

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub file: Option<PathBuf>,
}
