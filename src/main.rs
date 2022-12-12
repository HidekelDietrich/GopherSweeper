mod tui;

use gophersweeper::*;
use std::error::Error;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(
    name = "GopherSweeper",
    author = "Hidekel D. <hidekeldietrich@gmail.com>",
    version = "1.0.6",
    about = "Blazing fast, terminal-based minesweeper; but instead of mines, you must avoid Golang devs.",
    long_about = None
)]
struct Cli {
    #[arg(short, value_enum, default_value_t = Difficulty::Rust)]
    difficulty: Difficulty,

    #[arg(short = 's', value_enum, default_value_t = FieldSize::RustBinary)]
    field_size: FieldSize
}

#[derive(Copy, Clone, ValueEnum)]
enum Difficulty {
    Python,
    Rust,
    Assembly
}

#[derive(Copy, Clone, ValueEnum)]
enum FieldSize {
    LuaVM,
    RustBinary,
    ElectronApp
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let (field_width, field_height) = match cli.field_size {
        FieldSize::LuaVM => (10, 9),
        FieldSize::RustBinary => (15, 14),
        FieldSize::ElectronApp => (20, 18)
    };

    let gophers = match cli.difficulty {
        Difficulty::Python => (field_width * field_height) as f64 * 0.1,
        Difficulty::Rust => (field_width * field_height) as f64 * 0.15,
        Difficulty::Assembly => (field_width * field_height) as f64 * 0.20
    }.ceil();

    tui::run(GopherSweeper::new(field_width, field_height, gophers as usize))?;
    Ok(())
}
