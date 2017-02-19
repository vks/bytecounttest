#[macro_use]
extern crate bencher;
extern crate bytecount;
#[macro_use]
extern crate lazy_static;

extern crate bytecounttest;

use std::fs::File;
use std::io::Read;
use bencher::Bencher;
use bytecount::{count, naive_count};
use bytecounttest::*;

lazy_static! {
    static ref DATA: String = {
        let mut f = File::open("src/lib.rs").unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        s
    };
}

const NL: u8 = '\n' as u8;

fn bench_count(bench: &mut Bencher) {
    bench.iter(|| {
        count(DATA.as_bytes(), NL);
    })
}

fn bench_naive_count(bench: &mut Bencher) {
    bench.iter(|| {
        naive_count(DATA.as_bytes(), NL);
    })
}

fn bench_newlinecount_basic(bench: &mut Bencher) {
    bench.iter(|| {
        newlinecount_basic(DATA.as_bytes());
    })
}

fn bench_newlinecount_memchr(bench: &mut Bencher) {
    bench.iter(|| {
        newlinecount_memchr(DATA.as_bytes());
    })
}

fn bench_newlinecount_swar(bench: &mut Bencher) {
    bench.iter(|| {
        newlinecount_swar(DATA.as_bytes());
    })
}

fn bench_newlinecount_avx(bench: &mut Bencher) {
    bench.iter(|| {
        newlinecount_avx(DATA.as_bytes());
    })
}

fn bench_newlinecount_avxuu(bench: &mut Bencher) {
    bench.iter(|| {
        newlinecount_avxuu(DATA.as_bytes());
    })
}

fn bench_newlinecount_avxu(bench: &mut Bencher) {
    bench.iter(|| {
        newlinecount_avxu(DATA.as_bytes());
    })
}

benchmark_group!(benches, bench_count, bench_naive_count, bench_newlinecount_basic, bench_newlinecount_memchr, bench_newlinecount_swar, bench_newlinecount_avx, bench_newlinecount_avxuu, bench_newlinecount_avxu);
benchmark_main!(benches);
