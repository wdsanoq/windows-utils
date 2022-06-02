#![windows_subsystem = "windows"]
extern crate user32;
extern crate winapi;

use clap::{Parser, Subcommand};
use std::mem;
use std::ptr::null_mut;
use winapi::shared::windef::{HWND, POINT, RECT, SIZE};
use winapi::um::winuser::{MONITORINFO, MONITOR_DEFAULTTONEAREST, SW_SHOW};

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
    #[clap(arg_required_else_help = true)]
    MoveToMonitor { monitor: i32 },
}

fn focus_region(x: i32, y: i32) {
    let point = POINT { x, y };
    let handle_at_point: HWND = unsafe { winapi::um::winuser::WindowFromPoint(point) };
    let parent_handle: HWND = unsafe { winapi::um::winuser::GetParent(handle_at_point) };
    let handle = if parent_handle == null_mut() {
        handle_at_point
    } else {
        parent_handle
    };
    unsafe {
        winapi::um::winuser::SetForegroundWindow(handle);
        winapi::um::winuser::ShowWindow(handle, SW_SHOW);
    }
}

fn move_to_monitor(monitor: i32) {
    let mut rect: RECT = unsafe { mem::zeroed() };
    let mut wsize = SIZE { cx: 0, cy: 0 };
    let mut monitor_info: MONITORINFO = unsafe { mem::zeroed() };
    monitor_info.cbSize = mem::size_of::<MONITORINFO>().try_into().unwrap();

    unsafe {
        let active_window_handle = winapi::um::winuser::GetForegroundWindow();
        winapi::um::winuser::GetWindowRect(active_window_handle, &mut rect);
        let aminfo =
            winapi::um::winuser::MonitorFromWindow(active_window_handle, MONITOR_DEFAULTTONEAREST);

        winapi::um::winuser::GetMonitorInfoW(aminfo, &mut monitor_info);

        wsize.cx = rect.right - rect.left;
        wsize.cy = rect.bottom - rect.top;
        // winapi::um::winuser::MoveWindow(
        //     active_window_handle,
        //     -1800,
        //     40,
        //     wsize.cx,
        //     wsize.cy,
        //     monitor,
        // );
    }
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::FocusAt { x, y } => focus_region(*x, *y),
        Commands::MoveToMonitor { monitor } => move_to_monitor(*monitor),
    }
}
