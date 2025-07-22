mod counter;
mod find_comment_category;
mod utils;

use clap::Parser;
use counter::count_lines;
use std::{fs::File, io::Write, path::PathBuf};

use crate::utils::OutputFormat;

/// Simple tool to count lines of code
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Path to file or directory
    #[arg(short, long)]
    path: PathBuf,

    /// Optional file extension filter (e.g., rs, py)
    #[arg(short, long)]
    ext: Option<String>,

    /// Output format: plain, json
    #[arg(short, long, default_value = "plain")]
    format: OutputFormat,

    /// Output file (only works with --format json)
    #[arg(short = 'o', long)]
    output: Option<PathBuf>,

    /// List of directories to exclude from counting
    #[arg(long, value_delimiter = ',')]
    exclude: Vec<PathBuf>,

    /// Output file (only works with --format json)
    #[arg(long)]
    comment_style_json_path: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let result = count_lines(&args.path, args.ext.as_deref(), &args.exclude, args.comment_style_json_path);

    match args.format {
        OutputFormat::Plain => {
            for (file, stats) in result {
                println!("{}:", file.display());
                println!("  Total:   {}", stats.total);
                println!("  Code:    {}", stats.code);
                println!("  Comment: {}", stats.comments);
                println!("  Blank:   {}\n", stats.blanks);
            }
        },
        OutputFormat::Json => {
            let json_vec: Vec<_> = result
                .iter()
                .map(|(path, stats)| serde_json::json!({
                    "file": path.display().to_string(),
                    "total": stats.total,
                    "code": stats.code,
                    "comments": stats.comments,
                    "blanks": stats.blanks,
                }))
                .collect();

            let output = match serde_json::to_string_pretty(&json_vec){
                Ok(r) => {
                    r
                },
                Err(e) => {
                    println!("Error: {:?}", e);
                    return;
                }
            };

            if let Some(out_path) = args.output {
                let mut file = File::create(out_path).expect("Could not create output file");
                file.write_all(output.as_bytes()).expect("Write failed");
            } else {
                println!("{output}");
            }
        }
    }
}
