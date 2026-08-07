use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: password-tool <salt> <plain-password>");
        std::process::exit(2);
    }

    let salt = &args[1];
    let password = &args[2];

    match hbb_common::config::generate_permanent_password_storage(password, salt) {
        Some(storage) => println!("{storage}"),
        None => {
            eprintln!("Failed to generate permanent password storage");
            std::process::exit(1);
        }
    }
}
