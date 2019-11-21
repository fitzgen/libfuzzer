#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data == b"banana!" {
        panic!("success!");
    }
});
