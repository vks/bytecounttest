# bytecounttest

This uses quickcheck to verify the two implementations offered by
[bytecount]() are compatible. It also includes [Daniel Lemire's C
implementation of newline counting algorithms](https://github.com/lemire/Code-used-on-Daniel-Lemire-s-blog/blob/master/2017/02/14/newlines.c)
and some benchmarks. To run them use `RUSTFLAGS="-C target-cpu=native"
cargo bench`. This will likely fail if your architecture does not support
AVX2.

## Example for benchmarks

```
test bench_count               ... bench:          29 ns/iter (+/- 1)
test bench_naive_count         ... bench:         415 ns/iter (+/- 6)
test bench_newlinecount_avx    ... bench:          55 ns/iter (+/- 0)
test bench_newlinecount_avxu   ... bench:          40 ns/iter (+/- 0)
test bench_newlinecount_avxuu  ... bench:          56 ns/iter (+/- 2)
test bench_newlinecount_basic  ... bench:       1,873 ns/iter (+/- 44)
test bench_newlinecount_memchr ... bench:         406 ns/iter (+/- 5)
test bench_newlinecount_swar   ... bench:         365 ns/iter (+/- 7)
```

## Example for a quickcheck failure

```
running 2 tests
test tests::issue_found_by_quickcheck ... FAILED
test tests::quickcheck ... FAILED

failures:

---- tests::issue_found_by_quickcheck stdout ----
    thread 'tests::issue_found_by_quickcheck' panicked at 'assertion failed: `(left == right)` (left: `2`, right: `1`)', src/lib.rs:33
    note: Run with `RUST_BACKTRACE=1` for a backtrace.

    ---- tests::quickcheck stdout ----
        thread 'tests::quickcheck' panicked at '[quickcheck] TEST FAILED. Arguments: ([0, 0, 89, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 89)', /home/steinberg/.cargo/registry/src/github.com-1ecc6299db9ec823/quickcheck-0.3.1/src/tester.rs:118


        failures:
            tests::issue_found_by_quickcheck
                tests::quickcheck

                test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured

                error: test failed
```
