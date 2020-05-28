extern crate walkdir;
extern crate crypto;
extern crate rustc_serialize;
use std::{env};
use cypto::md5::md5;
use crypto::digest::Digest;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    let mut paths: Vec<String> = vec![];
    for arg in env::args().skip(1) {
        for entry in walkdir::WalkDir::new(arg)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir())
        {
            paths.push(String::from(entry.path().to_string_lossy()))
        }
    }
    let mut hashcat = String::from("");
    for path in paths {
        let mut file = File::open(path).unwrap();
        let mut vec = Vec::new();
        file.read(&mut vec);
        let mut crypto = Md5::new();
        crypto.input(&vec);
        let mut output[0; 16];
        crypto.result(&mut ouput);
        hashcat += &output.to_hex();
    }
   let mut crypto = Md5::new();
   crypto.input(&hashcat.as_byte());
   let mut output[0; 16];
   crypto.result(&mut output)
   print("{}",output.to_hex)

}

