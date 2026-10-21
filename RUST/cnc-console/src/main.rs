//! cnc-console binary entry point.

use clap::Parser;
use cnc_console::ConsoleConfig;

#[derive(Parser, Debug)]
#[command(name = "cnc-console")]
#[command(about = "C&C Console - Real-time Log and Event Monitor")]
struct Args {
    #[arg(long, default_value_t = 1200)]
    width: u32,

    #[arg(long, default_value_t = 800)]
    height: u32,

    #[arg(long, default_value = "C&C Console - Log & Event Monitor")]
    title: String,

    #[arg(long, default_value = "info")]
    log_level: String,

    #[arg(long)]
    headless: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config = ConsoleConfig {
        window_width: args.width as f32,
        window_height: args.height as f32,
        window_title: args.title,
        log_level: args.log_level,
        headless: args.headless,
        ..ConsoleConfig::default()
    };
    cnc_console::run_console(config)
}
