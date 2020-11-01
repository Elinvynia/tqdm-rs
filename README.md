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
}
```
