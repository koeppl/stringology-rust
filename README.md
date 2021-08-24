# Stringology-Rust

This project contains some simple and easy-to-use tools for studying of strings whose characters are drawn from the byte alphabet.
It currently consists of 
 - analytic tools
   - `lyndonfactorization` : counts the number of Lyndon factors. Outputs all ending positions of Lyndon factors when setting the environment variable `RUST_LOG=debug`
   - `count_r` : counts the number of runs in the BWT obtained by the suffix array
   - `count_sigma` : counts the number of different characters
   - `count_z` : counts the number of overlapping LZ77 factors
   - `entropy` : counts the k-th order empirical entropy
 - generators
   - `thuemorse` : computes the n-th [Thue-Morse word](https://oeis.org/A010060)
   - `fibonacci` : computes the n-th [Fibonacci word](https://oeis.org/A003849)
   - `perioddoubling` computes the n-th [period-doubling sequence](https://oeis.org/A096268)
   - `debruijn` : computes a binary de Bruijin word of order n

## Usage

Compile and run with `cargo` of `rust`-lang:

```
cargo build
cargo run --bin count_sigma -- --file ./data/tudocomp/einstein.en.txt
```

```
cargo run --bin fibonacci 5
```


Datasets can be found at http://dolomit.cs.tu-dortmund.de/tudocomp/

The output format of the analytic tools is compatible with [sqlplot](https://github.com/koeppl/sqlplot).
