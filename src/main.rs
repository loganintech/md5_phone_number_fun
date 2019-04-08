#![allow(clippy::inconsistent_digit_grouping)]

use hashbrown::HashSet;
use md5::compute;

fn main() {
    let mut set = HashSet::with_capacity(1_999_999_9999 - 1_000_000_0000);
    (1_000_000_0000u128..1_999_999_9999u128).for_each(move |i| {
        let bytes: [u8; 16] = compute(i.to_be_bytes()).into();
        if !set.insert(bytes) {
            eprintln!("Collision Detected!");
        }
    });
}
