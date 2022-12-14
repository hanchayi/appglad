use std::{env::args, path::PathBuf};

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
}
