fn main() {
    let root = cmd::init();
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        root.print_help();
        return;
    }

    if let Err(e) = root.execute(args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

mod cmd;
mod tasks;
mod utils;
