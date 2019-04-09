use std::fs::OpenOptions;
use std::io::{BufReader, BufRead, Read};

fn main() {
    let file = OpenOptions::new()
        .read(true)
        .open("./collisions.txt")
        .unwrap();


    let file = BufReader::new(file);
    let line_iter = file.lines().filter_map(Result::ok);
    // let mut line_count = 0;
    for source in line_iter {
        let file_two = OpenOptions::new()
            .read(true)
            .open("./collisions.txt")
            .unwrap();
        let split_source = source.split(':').collect::<Vec<_>>();
        for check in BufReader::new(file_two).lines().filter_map(Result::ok) {
            let split_check = check.split(':').collect::<Vec<_>>();
            if split_source[0] != split_check[0] && split_source[1] == split_check[1] {
                println!("Collision Detected!!");
                println!("{} - {} with hash {}", split_source[0], split_source[1], split_check[0]);
            }
        }
    }

}
