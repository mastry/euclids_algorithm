# Euclid's Algorithm Code

This is the supporting code for [my blog post](https://jeffmastry.dev/posts/computing-the-greatest-common-divisor) on calculating the greatest common divisor of two integers. It contains the Rust algorithm code and supporting benchmarks (also in Rust).

## Usage

> I'm assuming you already have Rust nightly installed and working. Nightly is required for the benchmarking feature.

### Run all benchmarks at once

```sh
cargo bench
```

### Run benchmarks for the "naïve" implementation

```sh
cargo bench -- naive
```

Sample output:

```sh
running 8 tests
test tests::naive_1_digit  ... bench:           0 ns/iter (+/- 0)
test tests::naive_2_digit  ... bench:           0 ns/iter (+/- 0)
test tests::naive_3_digit  ... bench:         278 ns/iter (+/- 64)
test tests::naive_4_digit  ... bench:       8,279 ns/iter (+/- 1,971)
test tests::naive_5_digit  ... bench:      46,684 ns/iter (+/- 7,123)
test tests::naive_6_digit  ... bench:     761,656 ns/iter (+/- 118,619)
test tests::naive_7_digit  ... bench:   4,050,820 ns/iter (+/- 32,193)
test tests::naive_8_digit  ... bench:  39,972,420 ns/iter (+/- 13,088,906)

test result: ok. 0 passed; 0 failed; 0 ignored; 8 measured; 8 filtered out; finished in 21.91s
```

### Run benchmarks for Euclid's Algorithm

```sh
cargo bench -- euclid
```

Sample output:

```sh
running 8 tests
test tests::euclid_1_digit ... bench:           4 ns/iter (+/- 0)
test tests::euclid_2_digit ... bench:           6 ns/iter (+/- 0)
test tests::euclid_3_digit ... bench:          12 ns/iter (+/- 0)
test tests::euclid_4_digit ... bench:          26 ns/iter (+/- 0)
test tests::euclid_5_digit ... bench:          21 ns/iter (+/- 1)
test tests::euclid_6_digit ... bench:          21 ns/iter (+/- 1)
test tests::euclid_7_digit ... bench:          38 ns/iter (+/- 4)
test tests::euclid_8_digit ... bench:          50 ns/iter (+/- 17)

test result: ok. 0 passed; 0 failed; 0 ignored; 8 measured; 8 filtered out; finished in 15.93s
```
