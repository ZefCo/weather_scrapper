extern crate serde;
extern crate serde_json;
extern crate serde_derive;

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

struct Location_Json {
    name: String,
    latt: f32,
    long: f32
}

pub fn read_file(path: PathBuf) {
    let mut f = File::open(path)
        .expect("Can't open file");
    
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Can not read file...");
}