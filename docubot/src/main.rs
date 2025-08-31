use std::env;
use std::process::exit;

const BANNER: &str = r"
 ____   __    ___  _  _  ____   __  ____
(    \ /  \  / __)/ )( \(  _ \ /  \(_  _)
 ) D ((  O )( (__ ) \/ ( ) _ ((  O ) )(
(____/ \__/  \___)\____/(____/ \__/ (__)
";

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(0);

    match path {
        Some(p) if !p.is_empty() => {}
        _ => {
            eprintln!("Failed to find suitable CLI commands... exiting");
            exit(1)
        }
    }
    print!("{}\n", BANNER);
    println!("Starting...")
}
