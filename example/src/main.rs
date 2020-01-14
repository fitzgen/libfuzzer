#![no_main]

use libfuzzer::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data == b"banana!" {
        panic!("success!");
    }
});
