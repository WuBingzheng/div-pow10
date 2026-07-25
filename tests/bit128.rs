use div_pow10::bit128::*;

const POWERS: [u128; 39] = [
    1,
    10_u128.pow(1),
    10_u128.pow(2),
    10_u128.pow(3),
    10_u128.pow(4),
    10_u128.pow(5),
    10_u128.pow(6),
    10_u128.pow(7),
    10_u128.pow(8),
    10_u128.pow(9),
    10_u128.pow(10),
    10_u128.pow(11),
    10_u128.pow(12),
    10_u128.pow(13),
    10_u128.pow(14),
    10_u128.pow(15),
    10_u128.pow(16),
    10_u128.pow(17),
    10_u128.pow(18),
    10_u128.pow(19),
    10_u128.pow(20),
    10_u128.pow(21),
    10_u128.pow(22),
    10_u128.pow(23),
    10_u128.pow(24),
    10_u128.pow(25),
    10_u128.pow(26),
    10_u128.pow(27),
    10_u128.pow(28),
    10_u128.pow(29),
    10_u128.pow(30),
    10_u128.pow(31),
    10_u128.pow(32),
    10_u128.pow(33),
    10_u128.pow(34),
    10_u128.pow(35),
    10_u128.pow(36),
    10_u128.pow(37),
    10_u128.pow(38),
];

#[test]
fn test_single() {
    let n = 123_u128;
    assert_eq!(div_single(n, 39), None);
    assert_eq!(div_single(n, 0), Some(n));

    const COUNT: u128 = 100000;
    const STEP: u128 = u128::MAX / COUNT;
    for i in 1..39 {
        let pow = POWERS[i as usize];
        for j in 0..COUNT {
            let n = j * STEP;
            assert_eq!(div_single(n, i), Some(n / pow));

            if n <= 2_u128.pow(127) {
                assert_eq!(unsafe { unchecked_div_single_r1b(n, i) }, n / pow);
            }
        }
    }
}

#[test]
fn test_double() {
    let n = 123_u128;
    assert_eq!(div_double(n, n, 39), None);
    assert_eq!(div_double(0, n, 0), Some((n, 0)));

    const COUNT: u128 = 1000; // enlarge this for more test
    const K_STEP: u128 = u128::MAX / COUNT;
    for i in 1..39 {
        let pow = POWERS[i as usize];
        let count = COUNT.min(pow);
        let step = pow / count;
        for j in 0..count {
            let high = j * step;

            for k in 0..COUNT {
                let low = k * K_STEP;

                let (q, r) = div_double(high, low, i).unwrap();

                let (p_high, p_low) = mul2(q, pow);
                let (p_low, carry) = p_low.overflowing_add(r);
                let p_high = p_high + carry as u128;

                assert_eq!(p_low, low);
                assert_eq!(p_high, high);
            }
        }
    }
}

// calculate: a * b => (mhigh,mlow)
const fn mul2(a: u128, b: u128) -> (u128, u128) {
    let (ahigh, alow) = (a >> 64, a & u64::MAX as u128);
    let (bhigh, blow) = (b >> 64, b & u64::MAX as u128);

    let (mid, carry1) = (alow * bhigh).overflowing_add(ahigh * blow);
    let (mlow, carry2) = (alow * blow).overflowing_add(mid << 64);
    let mhigh = ahigh * bhigh + (mid >> 64) + ((carry1 as u128) << 64) + carry2 as u128;
    (mhigh, mlow)
}
