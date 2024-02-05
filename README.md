# Stringology-Rust
![automatic test](https://github.com/koeppl/stringology-rust/actions/workflows/cargo.yml/badge.svg)

This project contains some simple and easy-to-use tools for studying of strings whose characters are drawn from the byte alphabet.
It currently consists of 
 - analytic tools
   - `lyndonfactorization` : counts the number of Lyndon factors. Outputs all ending positions of Lyndon factors when setting the environment variable `RUST_LOG=debug`
   - `count_r` : counts the number of runs in the BWT obtained by the suffix array
   - `count_sigma` : counts the number of different characters
   - `count_z` : counts the number of overlapping LZ77 factors
   - `entropy` : counts the k-th order empirical entropy
   - `entropykmer` : counts the k-th order empirical entropy via k-mers, `k \in [1..7]`
 - word generators with program `word`
   - `thuemorse` : computes the n-th [Thue-Morse word](https://oeis.org/A010060)
   - `fibonacci` : computes the n-th [Fibonacci word](https://oeis.org/A003849)
   - `perioddoubling` computes the n-th [period-doubling sequence](https://oeis.org/A096268)
   - `debruijn` : computes a binary de Bruijin word of order n
	 - `FibonacciLyndonFactor` : the n-th Lyndon factor of the infinite Fibonacci word
- `lyndonwords` to generate all Lyndon words up to a specific length for a given alphabet size
 - transforms
   - `reverse` : reverse the input byte-wise

## Usage

Compile and run with `cargo` of `rust`-lang:

```
cargo build
cargo run --bin count_sigma -- --file ./data/tudocomp/einstein.en.txt
```


compute the 5th Fibonacci word
```
cargo run --bin word -- -n fibonacci -k 5
```

Datasets can be found at http://dolomit.cs.tu-dortmund.de/tudocomp/

The output format of the analytic tools is compatible with [sqlplot](https://github.com/koeppl/sqlplot).

## CAVEATS

 - The BWT computation requires that the zero byte does not occur in your input. To enforce that, you can use the `escape` program to escape all zero bytes.
For instance, `escape -f 0 -t 255 -e 254 | count_r` escapes 0 with the escape byte 254 to byte 255, and pipes the input to `count_r` counting the number of BWT runs.


