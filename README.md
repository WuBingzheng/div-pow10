Fast division by powers of 10, in 64-bit and 128-bit.

Division is slow. However it can be optimized if the divisor is divided repeatly.

One of the use cases is divided by powers of 10. For example in a decimal crate.

This crate provides easy APIs for division by powers of 10. You do not need
to touch the magic numbers.


# Algorithm

We provides 2 kinds of division.

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
