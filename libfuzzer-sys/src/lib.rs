use std::{panic, process, slice};

extern "C" {
    // We do not actually cross the FFI boundary here.
    #[allow(improper_ctypes)]
    fn rust_fuzzer_test_input(input: &[u8]);
}

#[doc(hidden)]
#[export_name = "LLVMFuzzerTestOneInput"]
pub fn test_input_wrap(data: *const u8, size: usize) -> i32 {
    let test_input = panic::catch_unwind(|| unsafe {
        let data_slice = slice::from_raw_parts(data, size);
        rust_fuzzer_test_input(data_slice);
    });
    if test_input.err().is_some() {
        // hopefully the custom panic hook will be called before and abort the
        // process before the stack frames are unwinded.
        process::abort();
    }
    0
}

#[doc(hidden)]
#[export_name = "LLVMFuzzerInitialize"]
pub fn initialize(_argc: *const isize, _argv: *const *const *const u8) -> isize {
    // Registers a panic hook that aborts the process before unwinding.
    // It is useful to abort before unwinding so that the fuzzer will then be
    // able to analyse the process stack frames to tell different bugs appart.
    //
    // HACK / FIXME: it would be better to use `-C panic=abort` but it's currently
    // impossible to build code using compiler plugins with this flag.
    // We will be able to remove this code wheng
    // https://github.com/rust-lang/cargo/issues/5423 is fixed.
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        default_hook(panic_info);
        process::abort();
    }));
    0
}
