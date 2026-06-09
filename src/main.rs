mod model;
mod parser;
mod svg;

use clap::Parser;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Parser)]
#[command(
    name = "iso-cubeviz",
    version,
    about = "Rubik's cube isometric SVG renderer"
)]
struct Cli {
    #[arg(short = 's', long, conflicts_with = "state_file")]
    state: Option<String>,

    #[arg(long, conflicts_with = "state")]
    state_file: Option<PathBuf>,

    #[arg(long, default_value = "balanced")]
    view: String,

    #[arg(long, default_value = "normal")]
    border: String,

    #[arg(long, default_value = "true")]
    thickness: String,

    #[arg(long, default_value = "transparent")]
    background: String,

    #[arg(short = 'o', long)]
    output: Option<PathBuf>,
}

fn die(msg: &str) -> ! {
    eprintln!("error: {msg}");
    std::process::exit(1);
}

fn main() {
    let cli = Cli::parse();

    let state_text = match (&cli.state, &cli.state_file) {
        (Some(s), _) => s.clone(),
        (None, Some(path)) => {
            std::fs::read_to_string(path).unwrap_or_else(|e| die(&format!("cannot read file: {e}")))
        }
        (None, None) => die("one of --state or --state-file is required"),
    };

    let cube = parser::parse_state(&state_text).unwrap_or_else(|e| die(&e));

    let view = model::ViewMode::from_str(&cli.view).unwrap_or_else(|e| die(&e));
    let border = model::BorderStyle::from_str(&cli.border).unwrap_or_else(|e| die(&e));
    let thickness = match cli.thickness.as_str() {
        "true" => true,
        "false" => false,
        other => die(&format!(
            "invalid --thickness value: '{other}' (expected true or false)"
        )),
    };
    let background = model::BackgroundStyle::from_str(&cli.background).unwrap_or_else(|e| die(&e));

    let svg_output = svg::render(&cube, view, border, thickness, background);

    match &cli.output {
        Some(path) => {
            std::fs::write(path, &svg_output)
                .unwrap_or_else(|e| die(&format!("cannot write file: {e}")));
        }
        None => print!("{svg_output}"),
    }
}
