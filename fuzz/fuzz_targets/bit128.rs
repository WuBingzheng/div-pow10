#![no_main]

use libfuzzer_sys::fuzz_target;

use div_pow10::bit128::*;

#[derive(Debug, arbitrary::Arbitrary)]
struct Data {
    a: u128,
    b: u128,
    exp: u32,
}

fuzz_target!(|data: Data| {
    let a = data.a;
    let b = data.b;
    let exp = data.exp % 39;
    let pow = 10_u128.pow(exp);

    // single
    let q1 = div_single(a, exp).unwrap();
    let q2 = a / pow;
    assert_eq!(q1, q2);

    // double
    let (p1, p2) = mul2(a, b);
    match div_double(p1, p2, exp) {
        Some((q1, r1)) => {
            let (x1, x2) = mul2(q1, pow);
            let (y2, carry) = x2.overflowing_add(r1);
            let y1 = x1 + carry as u128;
            assert_eq!(p1, y1);
            assert_eq!(p2, y2);
        }
        None => assert!(p1 >= pow),
    }
});

// calculate: a * b => (mhigh,mlow)
const fn mul2(a: u128, b: u128) -> (u128, u128) {
    let (ahigh, alow) = (a >> 64, a & u64::MAX as u128);
    let (bhigh, blow) = (b >> 64, b & u64::MAX as u128);

    let (mid, carry1) = (alow * bhigh).overflowing_add(ahigh * blow);
    let (mlow, carry2) = (alow * blow).overflowing_add(mid << 64);
    let mhigh = ahigh * bhigh + (mid >> 64) + ((carry1 as u128) << 64) + carry2 as u128;
    (mhigh, mlow)
}
