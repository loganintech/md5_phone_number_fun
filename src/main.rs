#![allow(clippy::inconsistent_digit_grouping)]

use hex_slice::AsHex;
use md5::compute;
use rayon::prelude::*;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

fn main() {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("./collisions.txt")
        .unwrap();
    let mut file = BufWriter::new(file);

    let par_iter = (1_000_000_0000u128..1_999_999_9999u128)
        .into_par_iter()
        .map(|i| {
            let computed: [u8; 16] = compute(i.to_be_bytes()).into();
            computed
        });
    let res = par_iter.collect::<Vec<_>>();
    res.into_iter().for_each(|hex: [u8; 16]| {
        file.write_fmt(format_args!("{:x}\n", hex.as_hex()))
            .unwrap()
    });
}
