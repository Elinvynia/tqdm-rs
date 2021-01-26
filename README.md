[![ci-badge][]][ci] [![docs-badge][]][docs] [![crate-version]][crate-link]

# tqdm-rs

A simple progress bar library inspired by Python's `tqdm`.

## Sample Usage
```rust
fn main() {
    for _ in tqdm_rs::Tqdm::new(0..10) {
        tqdm_rs::write("Doing some work...\nOn multiple lines!");
        std::thread::sleep(std::time::Duration::from_secs(1));
        continue
    }

    // It is possible to use print, but it looks more clumsy!
    for _ in tqdm_rs::Tqdm::new(0..10) {
        println!("Doing some work...\nOn multiple lines!");
        std::thread::sleep(std::time::Duration::from_secs(1));
        continue
    }

    let mut tq = tqdm_rs::Tqdm::manual(100)
    for _ in 0..10 {
        println!("I am updated manually!")
        tq.update(10)
    }
}
```

[ci]: https://github.com/Elinvynia/tqdm-rs/actions?query=workflow%3ARust
[ci-badge]: https://img.shields.io/github/workflow/status/Elinvynia/tqdm-rs/Rust/master?style=flat-square
[docs]: https://docs.rs/tqdm-rs
[docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square
[crate-link]: https://crates.io/crates/tqdm-rs
[crate-version]: https://img.shields.io/crates/v/tqdm-rs.svg?style=flat-square
