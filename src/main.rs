#![windows_subsystem = "windows"]
extern crate user32;
extern crate winapi;

use clap::{Parser, Subcommand};
use winapi::shared::windef::POINT;
use winapi::um::winuser::SW_SHOW;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    /// Focus topmost window at x y
    #[clap(arg_required_else_help = true)]
    FocusAt { x: i32, y: i32 },
}

fn focus_region(x: i32, y: i32) {
    let point = POINT { x, y };
    unsafe {
        let left_window_handle = winapi::um::winuser::WindowFromPoint(point);
        winapi::um::winuser::SetForegroundWindow(left_window_handle);
        winapi::um::winuser::ShowWindow(left_window_handle, SW_SHOW);
    }
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::FocusAt { x, y } => focus_region(*x, *y),
    }
}
