# bytecounttest

This uses quickcheck to verify the two implementations offered by [bytecount]() are compatible.

## Example

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
