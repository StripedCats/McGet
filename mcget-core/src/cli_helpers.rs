pub use colored::*;
use {
    indicatif::ProgressBar,
};

pub type RResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn get_or<T>(value: Option<T>, message: &str) -> T {
    match value {
        Some(v) => {
            v
        },

        None => {
            eprintln!("[McGet] {}: {}", "Error".bold(), message.red().bold());
            std::process::exit(1);
        }
    }
}

pub trait Capitalizer {
    fn capitalize(self) -> String;
}
  
impl Capitalizer for String {
    fn capitalize(self) -> String {
        let mut c = self.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}

pub struct SingleProgressIndicator {
    pub bar: ProgressBar
}

impl SingleProgressIndicator {
    pub fn new() -> SingleProgressIndicator {
        let bar = ProgressBar::new(2);
        bar.inc(1);

        SingleProgressIndicator{bar: bar}
    }
}

impl Drop for SingleProgressIndicator {
    fn drop(&mut self) {
        self.bar.inc(1);
        self.bar.finish();
    }
}

#[inline(always)]
pub fn log(string: &str) {
    println!("[McGet] {}", string);
}
