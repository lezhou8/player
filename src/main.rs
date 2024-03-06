use std::env;
use std::thread;
use std::fs::{File, metadata};
use std::io::BufReader;
use std::time::Duration;
use rodio::{Decoder, Sink, OutputStream};

fn main() {
    let args: Vec<String> = env::args().collect();

    // checking args
    if args.len() < 2 {
        println!("Usage: {} <file1> <file2> ...", args[0]);
        std::process::exit(0);
    }

    if args.len() == 2 && &args[1] == "-v" {
        println!("v0.1.0");
        std::process::exit(0);
    }

    if args.len() == 2 && &args[1] == "-h" {
        println!("Usage: {} <file1> <file2> ...", args[0]);
        std::process::exit(0);
    }

    // checking file args
    for file_path in args.iter().skip(1) {
        if let Ok(metadata) = metadata(file_path) {
            if !metadata.is_file() {
                eprintln!("Error: {} is not a valid file.", file_path);
                std::process::exit(1);
            }
        } else {
            eprintln!("Error: {} does not exist.", file_path);
            std::process::exit(1);
        }
    }

    // set up audio stuff
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // start playing
    for file_path in args.iter().skip(1) {
        let file = BufReader::new(File::open(file_path).unwrap());
        let source = Decoder::new(file).unwrap();
        let _ = sink.append(source);
        sink.sleep_until_end();
        thread::sleep(Duration::from_secs(1));
    }
}
