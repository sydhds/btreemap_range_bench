# btreemap_range_bench

Bench BTreeMap find range of 2 diff impl 

Bench between: 
* find_bounds_range: using 2 BtreeMap.range() call
* find_bounds_custom: using 2 ifs + BTreeMap iterator

# Tests

Can run the test to check if the two functions behave the same:

    cargo test

# Run the benchmark

First, you need to generate fake data then run the bench:

    cargo run --example gen_fake_data
    cargo bench
    xdg-open target/criterion/find_bounds/report/index.html

Notes:
* Benchmark is run multiple time with data_{10/25/50/100}.json

# Results

We compare the 2 functions with 10, 25, 50, 100 entries (in BtreeMap), querying it with 20 random values:

![Results](https://github.com/sydhds/btreemap_range_bench/blob/main/results.svg?raw=true)
