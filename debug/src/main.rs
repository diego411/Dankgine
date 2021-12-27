use colored::*;
use std::io;

mod sketches;

fn main() {
    println!(
        "{}",
        "Please enter the id of the sketch you would like to render.".bright_yellow()
    );

    loop {
        let mut id = String::new();
        io::stdin()
            .read_line(&mut id)
            .expect("Failed to read input.");

        match id.trim() {
            "1" => {
                println!("{} {}", "Starting sketch with id: ".green(), id.green());
                sketches::sketch1::setup_and_draw_sketch();
                break;
            }
            _ => {
                println!(
                    "{}",
                    "Thats is not a known id. Please enter a valid id.".red()
                );
            }
        }
    }
}
