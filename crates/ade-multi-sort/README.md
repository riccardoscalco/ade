# Ade-multi-sort

`ade-multi-sort` is a utility crate designed for sorting data based on multiple criteria. It provides a flexible way to apply a series of sorting rules to a collection.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ade-multi-sort = "0.1.0"
```

## Usage Example

The `multi_sort` function takes a mutable slice of items and a slice of functions (metrics) to sort by. The sorting is applied sequentially based on the order of metrics provided.

```rust
use ade_multi_sort::multi_sort;

// Define a struct to sort. In this case, points with x and y coordinates.
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // A vector of Point structs.
    let mut points = vec![
        Point { x: 2, y: 3 },
        Point { x: 1, y: 5 },
        Point { x: 2, y: 1 },
        Point { x: 1, y: 2 },
    ];

    // Sort the points first by 'x' coordinate, and then by 'y' coordinate for ties.
    multi_sort(
        &mut points,
        &[
            Box::new(|p: &Point| p.x), // Primary sort key: x
            Box::new(|p: &Point| p.y), // Secondary sort key: y
        ],
    );

    // The expected sorted order.
    let expected = vec![
        Point { x: 1, y: 2 },
        Point { x: 1, y: 5 },
        Point { x: 2, y: 1 },
        Point { x: 2, y: 3 },
    ];

    // Print the sorted points and assert the result.
    println!("Sorted points: {:?}", points);
    assert_eq!(points, expected);
}
```

## Documentation

The complete documentation is available on [docs.rs](https://docs.rs/ade-multi-sort).

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.