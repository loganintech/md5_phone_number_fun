#![allow(clippy::inconsistent_digit_grouping)]

use hex_slice::AsHex;
use md5::compute;
use rayon::prelude::*;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

use std::sync::{Arc, Mutex};
fn main() {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("./collisions.txt")
        .unwrap();
    let file = BufWriter::new(file);
    let file = Arc::new(Mutex::new(file));

    (1_000_000_0000u128..1_999_999_9999u128)
        .into_par_iter()
        .for_each(|i| {
            let bytes: [u8; 16] = compute(i.to_be_bytes()).into();
            file.lock()
                .unwrap()
                .write_fmt(format_args!("{:x}\n", bytes.as_hex()))
                .unwrap();
        });
}
