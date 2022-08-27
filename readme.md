
A Rust library to generate random fruits in the form of a Array with fixed length passed by argument

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rand_fruits = ""0.1.1""
```

Add this to your `main.rs` and don't forget to generate some apples to keep Go Lang away 😆

```
use rand_fruits::generate_fruits;

fn main() {
    let arg = 5;
    let answer = generate_fruits(arg);
    assert_eq!(5, answer.len());
    println!("{:?}", answer);
}

```


# License

rand_fruits is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.

<!-- 
cargo doc --open
cargo test

cargo login token  /// generate token from crates.io -->