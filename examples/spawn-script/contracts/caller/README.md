# caller

TODO: Write this readme

*This contract was bootstrapped with [ckb-script-templates].*

[ckb-script-templates]: https://github.com/cryptape/ckb-script-templates



failures:

---- tests::test_spawn stdout ----
[contract debug] Enter caller contract!
[contract debug] Enter callee contract!
[contract debug] r is ok? true
[contract debug] i is ? 1
thread 'tests::test_spawn' panicked at tests/src/tests.rs:71:50:
called `Result::unwrap_err()` on an `Ok` value: 123799
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


