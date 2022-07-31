#![feature(test)]
extern crate test;

/// A simple GCD calculator that checks every possible divisor
pub fn gcd_naive(a: u128, b: u128) -> u128 {
    // The GCD can't be greater than the smallest input
    let max_gcd = if a < b { a } else { b };

    // We start with the divisor that always works
    let mut gcd = 1;

    // Now we check every number in our range of
    //    possible divisors and keep the largest
    for div in 2..max_gcd + 1 {
        if a % div == 0 && b % div == 0 {
            gcd = div;
        }
    }
    return gcd;
}

/// Calculates the GCD with Euclid's Algorithm
pub fn gcd(a: u128, b: u128) -> u128 {
    // x should be the larger of (a,b) and y the smaller
    let (mut x, mut y) = if a > b { (a, b) } else { (b, a) };

    // euclid's algorithm
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }

    return x;
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn naive_1_digit(b: &mut Bencher) {
        b.iter(|| gcd_naive(2, 9));
    }

    #[bench]
    fn euclid_1_digit(b: &mut Bencher) {
        b.iter(|| gcd(2, 9));
    }

    #[bench]
    fn naive_2_digit(b: &mut Bencher) {
        b.iter(|| gcd_naive(18, 80));
    }

    #[bench]
    fn euclid_2_digit(b: &mut Bencher) {
        b.iter(|| gcd(18, 80));
    }

    #[bench]
    fn naive_3_digit(b: &mut Bencher) {
        b.iter(|| gcd_naive(207, 510));
    }

    #[bench]
    fn euclid_3_digit(b: &mut Bencher) {
        b.iter(|| gcd(207, 510));
    }

    #[bench]
    fn naive_4_digit(b: &mut Bencher) {
        b.iter(|| gcd_naive(2238, 5107));
    }

    #[bench]
    fn euclid_4_digit(b: &mut Bencher) {
        b.iter(|| gcd(2238, 5107));
    }

    #[bench]
    fn naive_5_digit(b: &mut Bencher) {
        b.iter(|| gcd_naive(42238, 15107));
    }

    #[bench]
    fn euclid_5_digit(b: &mut Bencher) {
        b.iter(|| gcd(42238, 15107));
    }

    #[bench]
    fn naive_6_digit(b: &mut Bencher) {
        b.iter(|| gcd_naive(492238, 615107));
    }

    #[bench]
    fn euclid_6_digit(b: &mut Bencher) {
        b.iter(|| gcd(492238, 615107));
    }

    #[bench]
    fn naive_7_digit(b: &mut Bencher) {
        b.iter(|| gcd_naive(4392238, 2615107));
    }

    #[bench]
    fn euclid_7_digit(b: &mut Bencher) {
        b.iter(|| gcd(4392238, 2615107));
    }

    #[bench]
    fn naive_8_digit(b: &mut Bencher) {
        b.iter(|| gcd_naive(43922398, 26151047));
    }

    #[bench]
    fn euclid_8_digit(b: &mut Bencher) {
        b.iter(|| gcd(43922398, 26151047));
    }
}
