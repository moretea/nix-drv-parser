extern crate nix_drv_parser;

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        println!("{:?}", nix_drv_parser::parse_file(&path));
    } else {
        println!("No path provided!");
        std::process::exit(1)
    }
}
