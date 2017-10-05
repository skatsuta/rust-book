use std::io;
use std::io::Read;
use std::io::ErrorKind;
use std::fs::File;

const FILE_PATH: &str = "/tmp/hello.txt";

fn main() {
    {
        let f = File::open(FILE_PATH);

        let f = match f {
            Ok(file) => file,
            Err(ref err) if err.kind() == ErrorKind::NotFound => {
                match File::create(FILE_PATH) {
                    Ok(fc) => fc,
                    Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
                }
            }
            Err(err) => panic!("There was a problem opening the file: {:?}", err),
        };

        println!("{:?}", f);
    }
    {
        let f = File::open(FILE_PATH).unwrap();
        println!("{:?}", f);

        let f = File::open(FILE_PATH).expect(&format!("Failed to open {}", FILE_PATH));
        println!("{:?}", f);
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(FILE_PATH)?.read_to_string(&mut s)?;
    Ok(s)
}
