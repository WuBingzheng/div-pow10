#![no_main]

use libfuzzer_sys::fuzz_target;

use div_pow10::bit64::*;

#[derive(Debug, arbitrary::Arbitrary)]
struct Data {
    a: u64,
    b: u64,
    exp: u32,
}

fuzz_target!(|data: Data| {
    let a = data.a;
    let b = data.b;
    let exp = data.exp % 20;
    let pow = 10_u64.pow(exp);

    // single
    let q1 = div_single(a, exp).unwrap();
    let q2 = a / pow;
    assert_eq!(q1, q2);

    // double
    let p = a as u128 * b as u128;
    let q2 = p / pow as u128;
    let r2 = p % pow as u128;

    match div_double(p, exp) {
        Some((q1, r1)) => {
            assert_eq!(q1, u64::try_from(q2).unwrap());
            assert_eq!(r1, u64::try_from(r2).unwrap());
        }
        None => assert!(q2 > u64::MAX as u128),
    }
});
