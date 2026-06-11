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

    #[arg(long)]
    view: Option<String>,

    #[arg(long)]
    border: Option<String>,

    #[arg(long)]
    background: Option<String>,

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
    let opts = parser::parse_options(&state_text);

    // 優先順位: CLI 明示指定 > state file 記述 > 組み込みデフォルト
    let view_s = cli.view.or(opts.view).unwrap_or_else(|| "front".into());
    let border_s = cli
        .border
        .or(opts.border)
        .unwrap_or_else(|| "normal".into());
    let background_s = cli
        .background
        .or(opts.background)
        .unwrap_or_else(|| "transparent".into());

    let view = model::ViewMode::from_str(&view_s).unwrap_or_else(|e| die(&e));
    let border = model::BorderStyle::from_str(&border_s).unwrap_or_else(|e| die(&e));
    let background = model::BackgroundStyle::from_str(&background_s).unwrap_or_else(|e| die(&e));

    let svg_output = svg::render(&cube, view, border, background);

    match &cli.output {
        Some(path) => {
            std::fs::write(path, &svg_output)
                .unwrap_or_else(|e| die(&format!("cannot write file: {e}")));
        }
        None => print!("{svg_output}"),
    }
}
