// Haski - Oscar
// The use of this is restricted to only the authors

pub mod show {
    use colored::Colorize;

    pub fn print(origin: &str, data: &String) { println!("{} {}", format!("({})", origin).bold().yellow(), data) }
    pub fn printError(origin: &str, data: &String) { println!("{} {}", format!("(ERROR at {})", origin).bold().red(), data) }
    pub fn printTitle(data: &str) { println!("{}", format!("--- {} ---", data).bright_white()) }
}