/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

//! Dump lron file. Allow validating the grammar.

extern crate lrcat;

use std::env;
use std::fs::File;
use std::io::Read;

/// Parse the lron file and output it's parsed content or an error.
/// @return an error in case of IO error.
fn dump_lron(filename: &str) -> std::io::Result<()> {
    let mut file = File::open(filename).expect("Unknown file");

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let o = lrcat::lron::Object::from_string(&buffer);
    match o {
        Ok(ref o) => {
            println!("Result: {:?}", o);
        }
        Err(e) => println!("Error parsing file {}: {:?}", filename, e),
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }

    for filename in args.iter() {
        if dump_lron(filename).is_err() {
            println!("Error dumping lron");
        }
    }
}
