use std::{env::args, path::PathBuf, fs::{self, File}, io::{BufReader, BufRead}, time::Instant};
use anyhow::{ Result, Context }; 
#[derive(Debug)]
struct Options {
    pattern: String,
    path: PathBuf, // 跨平台的文件路径
}

fn main() -> Result<()> {
    let pattern = args().nth(1).expect("no pattern ");
    let path = args().nth(2).expect("no path");
    let options: Options = Options {
        pattern,
        path: PathBuf::from(path),
    };
    println!("options {:?}", options);
    let now = Instant::now();
    let res = read_to_string(&options);
    println!("time {}", now.elapsed().as_micros());
    res
}

fn read_buffer(options: &Options) {
    let file = File::open(&options.path).expect("cound not read file");
    let lines = BufReader::new(file).lines();

    for line in lines {
        if let Ok(data) = line {
            if data.contains(&options.pattern) {
                println!("{}", data);
            }
        }
    }
}

fn read_to_string(options: &Options) -> Result<()> {
    let content = fs::read_to_string(&options.path)
        .with_context(|| { format!("invalid path {}", &options.path.to_string_lossy()) })?;

    for line in content.lines() {
        if line.contains(&options.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
    
}
