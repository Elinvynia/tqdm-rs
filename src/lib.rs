use std::io::Write;

pub struct Tqdm;

impl Tqdm {
    pub fn new<I: Iterator>(iter: I) -> TqdmAuto<I> {
        let total = iter.size_hint().0;
        TqdmAuto {iter, current: 0, total}
    }
    pub fn manual(total: usize) -> TqdmManual {
        TqdmManual {current: 0, total}
    }
}

pub fn writeln(text: &str) {
    std::thread::sleep(std::time::Duration::from_secs(1));
    let size = terminal_size::terminal_size().expect("Failed to get terminal dimensions.");
    let whitespace = " ".repeat((size.0.0 as usize).checked_sub(text.len()).unwrap_or(0));
    println!("\r{}{}", text, whitespace);
}

trait WriteCon {
    fn get_current_amount(&self) -> usize {0}
    fn get_total_amount(&self) -> usize {0}
    fn get_percentage(&self) -> usize {
        let fraction = self.get_current_amount() as f32 / self.get_total_amount() as f32;
        (fraction * 100f32).round() as usize
    }
    fn create_bar(&self) -> String {
        let percents = self.get_percentage();
        let current = self.get_current_amount();
        let total = self.get_total_amount();
        let length = total.to_string().len();
        format!("{:>3}% | progress bar | {:>length$}/{}", percents, current, total, length = length)
    }

    fn display(&self) {
        let bar = self.create_bar();
        let size = terminal_size::terminal_size().expect("Failed to get terminal dimensions.");
        let whitespace = " ".repeat(size.0.0 as usize);
        print!("\r{}\r{}", whitespace, bar);
        std::io::stdout().flush().ok();
    }
}

pub struct TqdmAuto<I: Iterator> {
    iter: I,
    current: usize,
    total: usize,
}

impl<I: Iterator> Iterator for TqdmAuto<I> {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == 0 {
            self.display();
        };
        let next = self.iter.next();
        if next.is_some() {
            self.current += 1;
            self.display();
            Some(())
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

pub struct TqdmManual {
    current: usize,
    total: usize,
}

impl TqdmManual {
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

