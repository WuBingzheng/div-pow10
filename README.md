Fast division by powers of 10, for both 64-bit and 128-bit unsigned  integers.

Division is slow. However it can be optimized if the divisor is constant.
A common use case is division by powers of 10, such as in decimal crates.

This crate provides easy APIs for division by powers of 10. You do not need
to work with the algorithms or magic numbers.


# Algorithm

This crate provides 2 kinds of division.

The first is single-word. The divident is as wide as the divisor. The are both
64-bit or 128-bit.

We use the algorithm in Granlund & Montgomery's paper: [Division by Invariant
Integers using Multiplication](https://gmplib.org/%7Etege/divcnst-pldi94.pdf).
Here is a good [tutorial](https://homepage.divms.uiowa.edu/%7Ejones/bcd/divide.html)
for this algorithm.

The second is double-word. The divident is twice wide as the divisor, and the
quotient is the same wide as the divisor. For example, the divident is 256-bit
while the divisor and the quotient are both 128-bit. Or the divident is 128-bit
while the divisor and the quotient are both 64-bit.

We use the algorithm in Moller and Granlund's paper: [Improved division by
invariant integers](https://gmplib.org/~tege/division-paper.pdf).


# Usage

Here we take 64-bit as example:

```rust
use div_pow10::bit64::{div_single, div_double};

// single-word: 1234 / 10.pow(2)
assert_eq!(div_single(1234, 2), Some(1234 / 100));

// double-word: dd128 / 10.pow(2)
let dd128 = u64::MAX as u128 * 100; // 128-bit dividend
assert_eq!(div_double(dd128, 2), Some((u64::MAX, 0))); // quotient and remainder

// double-word: dd128 / 10.pow(1), quotient overflows 64-bit, so get None
assert_eq!(div_double(dd128, 1), None);
```


# Performance

The performance improvement brought by this crate is highly dependent on the
CPU model. For example, on some Intel CPUs where the division instruction is
not well optimized, this crate may deliver up to a 10X performance boost.
However, on Apple M-series chips, the improvement tends to be minimal.
Therefore, it is strongly recommended that you run benchmarks on your own
machine to determine whether to adopt this crate.

To run the benchmark:

```bash
git clone https://github.com/WuBingzheng/div-pow10.git
cd div-pow10
cargo bench
```
