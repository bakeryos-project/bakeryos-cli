// ANSI Color and Styling System for BakeryOS CLI

pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";
pub const ITALIC: &str = "\x1b[3m";
pub const UNDERLINE: &str = "\x1b[4m";

pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[37m";
pub const GRAY: &str = "\x1b[90m";

// Text helpers
pub fn bold<T: std::fmt::Display>(text: T) -> String {
    format!("{BOLD}{text}{RESET}")
}

pub fn dim<T: std::fmt::Display>(text: T) -> String {
    format!("{DIM}{text}{RESET}")
}

pub fn green<T: std::fmt::Display>(text: T) -> String {
    format!("{GREEN}{text}{RESET}")
}

pub fn red<T: std::fmt::Display>(text: T) -> String {
    format!("{RED}{text}{RESET}")
}

pub fn yellow<T: std::fmt::Display>(text: T) -> String {
    format!("{YELLOW}{text}{RESET}")
}

pub fn cyan<T: std::fmt::Display>(text: T) -> String {
    format!("{CYAN}{text}{RESET}")
}

pub fn blue<T: std::fmt::Display>(text: T) -> String {
    format!("{BLUE}{text}{RESET}")
}

// Status Badges
pub fn pass_badge() -> String {
    format!("{BOLD}{GREEN}[PASS]{RESET}")
}

pub fn fail_badge() -> String {
    format!("{BOLD}{RED}[FAIL]{RESET}")
}

pub fn warn_badge() -> String {
    format!("{BOLD}{YELLOW}[WARN]{RESET}")
}

pub fn busy_badge() -> String {
    format!("{BOLD}{YELLOW}[BUSY]{RESET}")
}

pub fn info_badge() -> String {
    format!("{BOLD}{CYAN}[INFO]{RESET}")
}

// UI Components
pub fn print_header(title: &str, subtitle: Option<&str>) {
    println!("\n{BOLD}{CYAN}{title}{RESET}");
    if let Some(sub) = subtitle {
        println!("{DIM}{sub}{RESET}");
    }
    println!("{}\n", dim("─".repeat(56)));
}

pub fn print_section(title: &str) {
    println!("\n{BOLD}{BLUE}:: {title}{RESET}");
    println!("{}", dim("─".repeat(45)));
}

pub fn print_kv<T: std::fmt::Display>(key: &str, val: T) {
    println!("   {CYAN}├─{RESET} {BOLD}{key}:{RESET} {val}");
}

pub fn print_kv_last<T: std::fmt::Display>(key: &str, val: T) {
    println!("   {CYAN}└─{RESET} {BOLD}{key}:{RESET} {val}");
}

pub fn divider(len: usize) -> String {
    dim("─".repeat(len))
}

/// Generates an ANSI progress bar: [██████░░░░] 60.0%
pub fn progress_bar(percent: f64, width: usize) -> String {
    let clamped = percent.clamp(0.0, 100.0);
    let filled = ((clamped / 100.0) * width as f64).round() as usize;
    let empty = width.saturating_sub(filled);

    let bar_color = if clamped >= 85.0 {
        RED
    } else if clamped >= 70.0 {
        YELLOW
    } else {
        GREEN
    };

    let filled_str = "█".repeat(filled);
    let empty_str = "░".repeat(empty);

    format!("{bar_color}[{filled_str}{empty_str}]{RESET} {clamped:5.1}%")
}
