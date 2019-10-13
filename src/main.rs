extern crate ferris_says;

use ferris_says::say;
use std::io::{ stdout, BufWriter };

fn main() {
    let out = b"L'impalpable verve de l'Art, enfin revelee";
    let width = out.len();

    let mut writer = BufWriter::new(stdout());
    say(out, width, &mut writer).unwrap();
}