mod rot13;

use rot13::rot13;
use std::fs::File;
use std::io::{ Read, Write };
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    println!("Provide a file to convert.\nUsage: {} filename", args[0]);
  } else {
    let ref filename = args[1];
    match File::open(filename) {
      Ok(mut file) => {
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        let converted = rot13(&content);

        // create opens file as write-only, deletes existing content
        match File::create(filename) {
          Err(e) => panic!("Couldn't create {}: {}", filename, e),
          Ok(mut file) => {
            match file.write_all(converted.as_bytes()) {
              Err(e) => panic!("Couldn't write to {}: {}", filename, e),
              Ok(_) => println!("Converted {}!", filename)
            };
          }
        };
      },
      Err(e) => println!("Error opening file {}: {}", filename, e)
    };
  }

}
