use div_pow10::bit64::*;

const POWERS: [u64; 20] = [
    1,
    10_u64.pow(1),
    10_u64.pow(2),
    10_u64.pow(3),
    10_u64.pow(4),
    10_u64.pow(5),
    10_u64.pow(6),
    10_u64.pow(7),
    10_u64.pow(8),
    10_u64.pow(9),
    10_u64.pow(10),
    10_u64.pow(11),
    10_u64.pow(12),
    10_u64.pow(13),
    10_u64.pow(14),
    10_u64.pow(15),
    10_u64.pow(16),
    10_u64.pow(17),
    10_u64.pow(18),
    10_u64.pow(19),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single() {
        let n = 123_u64;
        assert_eq!(div_single(n, 20), None);
        assert_eq!(div_single(n, 0), Some(n));

        const COUNT: u64 = 100000;
        const STEP: u64 = u64::MAX / COUNT as u64;
        for i in 1..20 {
            let pow = POWERS[i as usize];
            for j in 0..COUNT {
                let n = j * STEP;
                assert_eq!(div_single(n, i), Some(n / pow));

                if i < 19 && n <= 2_u64.pow(63) {
                    assert_eq!(unsafe { unchecked_div_single_r1b(n, i) }, n / pow);
                }
            }
        }
    }

    #[test]
    fn test_double() {
        let n = 123_u128;
        assert_eq!(div_double(n, 20), None);
        assert_eq!(div_double(n, 0), Some((n as u64, 0)));

        const COUNT: u64 = 1000; // enlarge this for more test
        const K_STEP: u64 = u64::MAX / COUNT;
        for i in 1..20 {
            let pow = POWERS[i as usize];
            let count = COUNT.min(pow);
            let step = pow / count;
            for j in 0..count {
                let high = j * step;

                for k in 0..COUNT {
                    let low = k * K_STEP;

                    let n = ((high as u128) << 64) + low as u128;

                    let (q, r) = div_double(n, i).unwrap();

                    let p = q as u128 * pow as u128 + r as u128;

                    assert_eq!(p, n);
                }
            }
        }
    }
}
