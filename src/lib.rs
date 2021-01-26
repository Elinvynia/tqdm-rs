//! [![ci-badge][]][ci] [![docs-badge][]][docs] [![crate-version]][crate-link]
//!
//! A simple progress bar library inspired by Python's `tqdm`.
//!
//! ```rust
//! for _ in tqdm_rs::Tqdm::new(0..10) {
//!     tqdm_rs::write("Doing some work...\nOn multiple lines!");
//!     std::thread::sleep(std::time::Duration::from_secs(1));
//!     continue
//! }
//!
//! // It is possible to use print, but it looks more clumsy!
//! for _ in tqdm_rs::Tqdm::new(0..10) {
//!     println!("Doing some work...\nOn multiple lines!");
//!     std::thread::sleep(std::time::Duration::from_secs(1));
//!     continue
//! }
//!
//! let mut tq = tqdm_rs::Tqdm::manual(100)
//! for _ in 0..10 {
//!     println!("I am updated manually!")
//!     tq.update(10)
//! }
//! ```
//!
//! [ci]: https://github.com/Elinvynia/tqdm-rs/actions?query=workflow%3ARust
//! [ci-badge]: https://img.shields.io/github/workflow/status/Elinvynia/tqdm-rs/Rust/master?style=flat-square
//! [docs]: https://docs.rs/tqdm-rs
//! [docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square
//! [crate-link]: https://crates.io/crates/tqdm-rs
//! [crate-version]: https://img.shields.io/crates/v/tqdm-rs.svg?style=flat-square

#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref TEXT: Mutex<String> = Mutex::new(String::new());
}

#[derive(Debug)]
/// Empty struct used for creating one of the two tqdm types.
pub struct Tqdm;

impl Tqdm {
    #[allow(clippy::new_ret_no_self)]
    /// Creates a new `tqdm` instance which handles progress automatically.
    pub fn new<I: Iterator>(iter: I) -> TqdmAuto<I> {
        let total = iter.size_hint().0;
        TqdmAuto {
            iter,
            current: 0,
            total,
        }
    }

    /// Creates a new manual `tqdm` instance, providing control over the progress.
    pub fn manual(total: usize) -> TqdmManual {
        TqdmManual { current: 0, total }
    }
}

/// Properly handles writing text along with the progress bar.
/// `println!` works, but looks a little clumsy if the loop sleeps.
pub fn write(text: &str) {
    let mut msg = TEXT.lock().unwrap();
    *msg = String::from(text);
}

fn clear_previous_line() {
    print!("\x1b[1A");
    print!("\r");
    let size = terminal_size::terminal_size().expect("Unable to get terminal size.");
    let width = size.0 .0 as usize;
    print!("{}", " ".repeat(width));
    print!("\r");
}

trait WriteCon {
    fn get_current_amount(&self) -> usize {
        0
    }
    fn get_total_amount(&self) -> usize {
        0
    }
    fn get_percentage(&self) -> usize {
        let fraction = self.get_current_amount() as f32 / self.get_total_amount() as f32;
        (fraction * 100f32).round() as usize
    }
    fn create_bar(&self) -> String {
        let percents = self.get_percentage();
        let current = self.get_current_amount();
        let total = self.get_total_amount();
        let length = total.to_string().len();
        let mut bar = String::new();
        bar += "[";
        for x in 1..=20 {
            if x * 5 <= percents {
                bar += "#"
            } else {
                bar += " "
            }
        }
        bar += "]";
        format!(
            "{:>3}% {} {:>length$}/{}",
            percents,
            bar,
            current,
            total,
            length = length
        )
    }

    fn display(&self) {
        let bar = self.create_bar();
        let mut text = TEXT.lock().unwrap();

        if self.get_current_amount() == 0 {
            return;
        }

        let lines = text.matches('\n').count();
        if self.get_current_amount() == 1 && lines > 0 {
            for _ in 0..=lines + 1 {
                println!();
            }
        }
        for _ in 0..lines {
            clear_previous_line();
        }
        if lines > 0 {
            clear_previous_line();
            clear_previous_line();
        }

        if !text.is_empty() {
            println!("{}", text);
            *text = String::new();
        }

        println!("{}", bar);
    }
}

#[derive(Debug)]
/// Struct that handles progress automatically.
pub struct TqdmAuto<I: Iterator> {
    iter: I,
    current: usize,
    total: usize,
}

impl<I: Iterator> Iterator for TqdmAuto<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next();
        if next.is_some() {
            self.display();
            self.current += 1;
            next
        } else {
            self.display();
            None
        }
    }
}

impl<I: Iterator> WriteCon for TqdmAuto<I> {
    fn get_current_amount(&self) -> usize {
        self.current
    }
    fn get_total_amount(&self) -> usize {
        self.total
    }
}
#[derive(Debug)]
/// If you want to manually update the progress, use this struct.
pub struct TqdmManual {
    current: usize,
    total: usize,
}

impl TqdmManual {
    /// Updates the progress manually and displays the progress bar.
    pub fn update(mut self, num: usize) {
        self.current = std::cmp::min(self.total, self.current + num);
        self.display();
    }
}

impl WriteCon for TqdmManual {
    fn get_current_amount(&self) -> usize {
        self.current
    }
    fn get_total_amount(&self) -> usize {
        self.total
    }
}
