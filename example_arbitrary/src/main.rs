#![no_main]

use libfuzzer::{arbitrary::Arbitrary, fuzz_target};

#[derive(Arbitrary, Debug)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

fuzz_target!(|rgb: Rgb| {
    if rgb.r < rgb.g {
        if rgb.g < rgb.b {
            panic!("success: r < g < b!");
        }
    }
});
