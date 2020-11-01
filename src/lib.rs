use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref TEXT: Mutex<String> = Mutex::new(String::new());
}

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

pub fn write(text: &str) {
    let mut msg = TEXT.lock().unwrap();
    *msg = String::from(text);
}

fn clear_previous_line() {
    print!("\x1b[1A");
    print!("\r");
    let size = terminal_size::terminal_size().expect("Unable to get terminal size.");
    let width = size.0.0 as usize;
    print!("{}", " ".repeat(width));
    print!("\r");
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
        let mut text = TEXT.lock().unwrap();

        if self.get_current_amount() != 0 {
            let lines = text.matches("\n").count();
            for _ in 1..=lines {
                clear_previous_line();
            }
            clear_previous_line();
            clear_previous_line();
        }

        if !text.is_empty() {
            println!("{}", text);
            *text = String::new();
        } else {
            println!("");
        }

        println!("{}", bar);
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
        let next = self.iter.next();
        if next.is_some() {
            self.display();
            self.current += 1;
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

