#![allow(clippy::inconsistent_digit_grouping)]

use hex_slice::AsHex;
use md5::compute;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

fn main() {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("./collisions.txt")
        .unwrap();
    let mut file = BufWriter::new(file);

    (1_000_000_0000u128..1_999_999_9999u128).for_each(move |i| {
        let bytes: [u8; 16] = compute(i.to_be_bytes()).into();
        file.write_fmt(format_args!("{:x}\n", bytes.as_hex())).unwrap();
    });
}
