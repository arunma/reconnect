# reconnect - comparison/reconciliation tool built on Rust

Inspired by the usage of Merkle trees by Cassandra for detecting anti-entropy, this is an attempt to use Merkle trees for performing quick and efficient comparison of two datasets.  

This project has the following additives, from the Python version of the tool :

1. Redesigned the Merkle Tree to be an array (like the heap array) instead of a first class datastucture, to optimize on locality.
2. Enhanced the code to support CSVs. Has abstractions to extend to other formats.
3. The current implementation stores the hashes (32 bit) in memory.  However, the underlying data could be stored in an external store (database/file). Also, swapped Adler with murmur3 for speed.
4. A sample test of 1 million * 1 million records (6 columns) ran in ~22 seconds. The code is currently single threaded and runs on a single machine.  

## Usage

```
./reconnect -l <path to first file> -r <path to second file>
```

**Example:**
```
./reconnect -l ./fixtures/test_person1.csv -r ./fixtures/test_person2.csv
```

## Diff output

The output file would be written as `diff.csv` on the root location of your command.

## Build and Run
If you would like to run the program inside `cargo`, the command is 

```
cargo run -- -l ./fixtures/test_demographics1.csv -r ./fixtures/test_demographics2.csv
```

Alternatively (recommended), you could build the optimized binary for release using

```
cargo build --release 
```
(The binary would be inside your `target/release` folder)

## Complexity

As against traditional hash/tree based comparisons where the time complexity needs to be calculated based on input, this is an example of [output sensitive algorithm](https://en.wikipedia.org/wiki/Output-sensitive_algorithm) where the complexity is `O(dh)`, `d` being the number of leaf node differences and `h` being the height of the tree.

## Rust experts

Would greatly appreciate your feedback on idiomatic usage, best practices and optimizations.
