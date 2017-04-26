#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate bytecount;

fuzz_target!(|data: &[u8]| {
    assert_eq!(bytecount::count(data, 0),
               bytecount::naive_count(data, 0));
});
