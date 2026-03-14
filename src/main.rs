mod color;
mod download;

use clap::{Parser, ValueHint};
use std::process::exit;

#[derive(Parser, Debug)]
#[command(name = "dw")]
#[command(version = "0.3.4")]
#[command(about = "A simple downloader written in Rust with an awesome progressbar")]
struct Args {
    #[arg(help = "URL of the file", value_hint = ValueHint::Url)]
    url: String,

    #[arg(help = "Target filepath (existing directories will be treated as the target location)", default_value = None)]
    des: Option<String>,

    #[arg(short, long, help = "Overwrite if the file already exists", conflicts_with = "resume")]
    force: bool,

    #[arg(short = 'c', long, help = "Resume failed or cancelled download")]
    resume: bool,

    #[arg(short, long, help = "Print the filepath to stdout after downloading")]
    echo: bool,

    #[arg(short, long, help = "Suppress filesize and progress info")]
    quiet: bool,

    #[arg(short, long, default_value = "false", help = "Download files in batch from a file with URLs")]
    batch: bool,

    #[arg(long, default_value = None, help = "Icon indicating the percentage done")]
    done: Option<String>,

    #[arg(long, default_value = None, help = "Icon indicating the percentage remaining")]
    left: Option<String>,

    #[arg(long, default_value = None, help = "Icon indicating the current percentage in the progress bar")]
    current: Option<String>,

    #[arg(long, default_value = "", help = "Color for the done percentage icon")]
    color_done: String,

    #[arg(long, default_value = "", help = "Color for the remaining percentage icon")]
    color_left: String,

    #[arg(long, default_value = "", help = "Color for the current indicator icon")]
    color_current: String,

    #[arg(long, default_value = "|", help = "Icon for the border of the progress bar")]
    icon_border: String,
}

fn main() {
    let args = Args::parse();

    let color_engine = color::ShellColor::new();

    if !args.color_done.is_empty() && !color_engine.is_valid_color(&args.color_done) {
        eprintln!("invalid value passed for `color_done`");
        exit(1);
    }
    if !args.color_left.is_empty() && !color_engine.is_valid_color(&args.color_left) {
        eprintln!("invalid value passed for `color_left`");
        exit(1);
    }
    if !args.color_current.is_empty() && !color_engine.is_valid_color(&args.color_current) {
        eprintln!("invalid value passed for `color_current`");
        exit(1);
    }

    let mut downloader = download::Download::new(
        args.url,
        args.des,
        args.force,
        args.resume,
        args.echo,
        args.quiet,
        args.batch,
        args.done.unwrap_or_else(|| "▓".to_string()),
        args.left.unwrap_or_else(|| "░".to_string()),
        args.current.unwrap_or_else(|| "▓".to_string()),
        args.icon_border,
        args.color_done,
        args.color_left,
        args.color_current,
        color_engine,
    );

    let success = downloader.download();

    if success && args.echo {
        println!("{}", downloader.get_destination());
    }
}
