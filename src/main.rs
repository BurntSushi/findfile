use std::fs;
//use std::fs::{File, OpenOptions};
use std::fs::File;
use std::io;
use std::io::prelude::*;
//use std::os::unix;
use std::path::Path;

// A simple implementation of `% cat path`
fn cat(path: &Path) -> io::Result<String> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn main() {
    println!("`cat poem.txt`");
    match cat(&Path::new("poem.txt")) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(s) => println!("> {}", s),
    }

    println!("`ls /Users/annaliao/mytest`");
    // Read the contents of a directory, returns `io::Result<Vec<Path>>`
    match fs::read_dir("/Users/annaliao/mytest") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => for path in paths {
            println!("> {:?}", path.unwrap().path());
        },
    }

    for path in paths {
        match cat(&Path::new(path)) {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(s) => println!("> {}", s),
    }
        }
    }

}
