use flate2::read::GzDecoder;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::process;

fn main() {
    let log_path = Path::new("log_export_2017");
    let files = fs::read_dir(log_path).unwrap();

    for read_file in files {
        match read_file {
            Ok(file) => {
                let file = fs::File::open(file.path()).unwrap();
                let mut gz = GzDecoder::new(file);
                let mut string = String::new();

                gz.read_to_string(&mut string).unwrap();

                let lines = string.lines().filter(|line| line.contains("logged"));
                for line in lines {
                    println!("{}", line)
                }
            }
            Err(_) => process::exit(0),
        }
    }
}
