use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

pub fn error_main(do_panic: bool) {
    if do_panic {
        panic!("crash and burn");
        println!("unreachable");
    }

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file: {:?}", e),
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        }
    };

    let read_name = read_username_from_file();
    println!("read_name: {:?}", read_name);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
