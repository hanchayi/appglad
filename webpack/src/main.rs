use std::{env::args, path::PathBuf, fs::{self, File}, io::{BufReader, BufRead}, time::Instant};

#[derive(Debug)]
struct Options {
    pattern: String,
    path: PathBuf, // 跨平台的文件路径
}

fn main() {
    let pattern = args().nth(1).expect("no pattern ");
    let path = args().nth(2).expect("no path");
    let options: Options = Options {
        pattern,
        path: PathBuf::from(path),
    };
    println!("options {:?}", options);
    let now = Instant::now();
    read_buffer(&options);
    println!("time {}", now.elapsed().as_micros());
}

fn read_buffer(options: &Options) {
    let file = File::open(&options.path).expect("cound not read file");
    let lines = BufReader::new(file).lines();

    for line in lines {
        if let Ok(data) = line {
            println!("{}", data)
        }
    }
}

fn read_to_string(options: &Options) {
    let content = fs::read_to_string(&options.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&options.pattern) {
            println!("{}", line);
        }
    }
}
