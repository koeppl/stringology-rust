# Stringology-Rust

This project contains some simple and easy-to-use tools for studying of strings whose characters are drawn from the byte alphabet.
It currently consists of 
 - anaytic tools
   - `count_r` : counts the number of runs in the BWT
   - `count_sigma` : counts the number of different characters
 - generators
   - `thuemorse` : computes the n-th Thue morse code
   - `fibonacci` : computes the n-th Fibonacci word

## Usage

Compile and run with `cargo` of `rust`-lang:

```
cargo build
cargo run --bin count_sigma -- --file ./data/tudocomp/einstein.en.txt
```

Datasets can be found at http://dolomit.cs.tu-dortmund.de/tudocomp/
