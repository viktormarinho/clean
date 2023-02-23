use colored::Colorize;

pub trait BeautifulErrors<T> {
    fn expect_or_err(self, msg: &str) -> T;
}

impl<T> BeautifulErrors<T> for Option<T> {
    fn expect_or_err(self, msg: &str) -> T {
        match self {
            Some(val) => val,
            None => {
                println!("{}", msg.red().bold());
                std::process::abort();
            }
        }
    }
}

impl<T, E> BeautifulErrors<T> for Result<T, E> {
    fn expect_or_err(self, msg: &str) -> T {
        match self {
            Ok(val) => val,
            Err(_) => {
                println!("{}", msg.red().bold());
                std::process::abort();
            }
        }
    }
}

pub fn panic_err(msg: &str) -> ! {
    println!("{}", msg.red().bold());
    std::process::abort();
}