## What is Serde?

Serde is a framework for serializing and deserializing Rust data structures efficiently and generically.

The Serde ecosystem consists of data structures that know how to serialize and deserialize themselves along with data formats that know how to serialize and deserialize other things.

Serde provides the layer by which these two groups interact with each other, allowing any supported data structure to be serialized and deserialized using any supported data format.


### Design

Serde is built on Rust's powerful trait system.

* Serde provides the `Serialize` trait and `Deserialize` trait for data structures.

* Serde provides `derive` attributes, to generate implementations at compile time.

* Serde has no runtime overhead such as reflection or runtime type information.

* In many situations the interaction between data structure and data format can be completely optimized away by the Rust compiler.


### Demo of Serde

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Print {"x":1,"y":2}
    println!("{}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Print Point { x: 1, y: 2 }
    println!("{:?}", deserialized);
}
```
