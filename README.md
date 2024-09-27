# Fuel VM execution using zkVM

## Dependencies

- [Rust](https://rustup.rs/)
- [SP1](https://docs.succinct.xyz/getting-started/install.html)
- [RiscZero](https://dev.risczero.com/api/zkvm/install)

To enable GPU acceleration, you need to install the NVIDIA Container Toolkit and install Docker on your machine. You can
follow the setup instructions provided in the following link.

## Quick Start

This section explains how to generate a proof using either SP1 or RISC0 zkVMs.

### SP1

Build the program

```bash
cd sp1/program
cargo prove build
cd ../script
RUST_LOG=info cargo run --release -- --prove
```

To test the program, you can execute it without generating a proof and check how many cycles are used:

```bash
RUST_LOG=info cargo run --release -- --execute
```

To generate a proof, you should run the script with the --prove flag that will generate a proof:

```bash
RUST_LOG=info cargo run --release -- --prove
```

To enable AVX256 acceleration, you can set the RUSTFLAGS environment variable to include the following flags:

```bash
RUST_LOG=info RUSTFLAGS="-C target-cpu=native" cargo run --release -- --prove
```

For AVX512 acceleration, use:

```bash
RUST_LOG=info RUSTFLAGS="-C target-cpu=native -C target-feature=+avx512f" cargo run --release -- --prove
```

### RISC ZERO

Build the program

```bash
cd risc0/
cargo build --release
```

To test program, you can first execute program without generating a proof and get how many cycles are used.

```bash
RISC0_DEV_MODE=0 RUST_LOG=info cargo run --release
```

To generate a proof, you should run the following command:

```bash
RUST_LOG=info cargo run --release
```

## Evaluation result

This section describes the proof performance results for a single transaction using 30 million gas. Testing was
conducted using SP1 with AVX512 acceleration and RISC0 with GPU acceleration.

| zkVM  | Cycles        | Time                                |
|-------|---------------|-------------------------------------|
| SP1   | 4_683_685_830 | 9 hours, 16 minutes and 30 seconds  |
| RISC0 | 9_628_024_832 | 3 hours, 56 minutes and 45 seconds. |

The proof performance was tested on a machine with the following hardware:

* **OS**: Ubuntu 22.04.4 LTS
* **CPU**: AMD Ryzen 9 7950X 16-Core Processor
* **GPU**: NVIDIA RTX 4070 TI SUPER 16 GB
* **RAM**: 128 GB DDR5 5200 MHz