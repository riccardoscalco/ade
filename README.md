## Run tests

```
# Test all crates
cargo test --workspace

# or with nextest if available
cargo nextest run

# Test all crates in release mode
cargo test --workspace --release

# Test a specific crate
cargo test -p ade-graph

# Run a single test in a crate
cargo test -p ade-elementary-circuits test_elementary_circuits_complete_graph

# Run a single test in a crate showing output
cargo test -p ade-elementary-circuits test_elementary_circuits_complete_graph -- --nocapture

# Run a single test in a crate in release mode
cargo test -p ade-elementary-circuits test_elementary_circuits_complete_graph --release
```

## Build

```
# Development - fast to compile
cargo build --workspace

# Production - optimized for performance
cargo build --workspace --release

# Only a specific crate in release
cargo build -p ade-topological-sort --release
```

## Benchmark

```
# Run benchmarks within a crate
cargo bench --bench ade_elementary_circuits_bench

# Save baseline
cargo bench --bench ade_elementary_circuits_bench -- --save-baseline before_optimization

# Compare with baseline
cargo bench --bench ade_elementary_circuits_bench -- --baseline before_optimization
```
