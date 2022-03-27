// Haski - Oscar
// The use of this is restricted to only the authors

pub mod show {
    use colored::Colorize;

    pub fn print(origin: &str, data: &String) { println!("({}) {}", origin.yellow(), data) }
    pub fn printTitle(data: &str) { println!("{}", format!("--- {} ---", data).bright_white()) }
}