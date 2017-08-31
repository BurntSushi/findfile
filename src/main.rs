use std::error::Error;
use std::fs;
//use std::fs::{File, OpenOptions};
use std::fs::File;
use std::io;
use std::io::prelude::*;
//use std::os::unix;
use std::path::Path;
use std::process;

// A simple implementation of `% cat path`
fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s).map(|_| s)
}

fn try_main() -> Result<(), Box<Error>> {
    println!("`cat poem.txt`");
    let poem = cat(&Path::new("poem.txt"))?;
    println!("> {}", poem);

    println!("`ls /Users/annaliao/mytest`");
    // Read the contents of a directory, returns `io::Result<Vec<Path>>`
    for result in fs::read_dir("/Users/annaliao/mytest")? {
        let dirent = result?;
        println!("> {:?}", dirent.path());
        let contents = cat(&dirent.path().join("/name"))?;
        println!("> {}", contents);
    }
    Ok(())
}

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{}", err);
        process::exit(1);
    }
}
