#![no_main]
use libfuzzer_sys::fuzz_target;
extern crate syn;
fuzz_target!(|input: syn::__fuzz_struct_parse_file| {
    syn::__fuzz_parse_file(input);
});
