mod ffxiv;
mod templating;

use anyhow::{Context, Result};
use std::env;

fn main() -> Result<()> {
    // First argument should be the path to human.cmp
    let arg: Vec<String> = env::args().collect();
    let cmd = &arg[0];
    let cmp_path_opt = arg.get(1);
    let cmp_path = match cmp_path_opt {
        Some(p) => p,
        None => {
            println!("usage: {cmd} <path to human.cmp>");
            std::process::exit(1)
        }
    };

    let racial_colours =
        ffxiv::load_racial_colours(cmp_path).context("Error parsing human.cmp file")?;

    let html = templating::render_index(&racial_colours).context("Error rendering HTML page")?;
    std::fs::write("index.html", html).context("Error writing HTML file")?;
    println!("Output HTML file");

    Ok(())
}
