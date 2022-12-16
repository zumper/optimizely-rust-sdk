
# Performance Test

A performance test to compare the SDK of different languages.
Each script will generate one million decisions for the flag `buy_button` from the example datafile.
The user IDs will start with the phrase "user" followed by a number from 0 to 999,999.
Note that the times also include reading the datafile from disk and initializing the SDK. However, this should only be a fraction of the total time for one million decisions.

## Instructions

To run the scripts for each language, use the following commands:

### Rust
```sh
cargo build --release
time target/release/performance-test
```

### Python
```sh
pip install -r requirements.txt
time python src/main.py
```

### JavaScript
```sh
npm install
time node src/main.js
```

### Go
```sh
go build -o bin/main src/main.go
time bin/main
```

## Results

See [home page](/../../)