// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error, thread::sleep, time::Duration};

use memory_stats::memory_stats;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    println!("########## Without window ##########");
    print_memory();

    draw_ui()?;

    println!("\n########## Without window (again) ##########");
    print_memory();

    println!("done!");
    Ok(())
}

fn draw_ui() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    println!("\n########## With window ##########");
    slint::invoke_from_event_loop(print_memory).unwrap();

    ui.run()?;
    Ok(())
}

fn print_memory() {
    if let Some(usage) = memory_stats() {
        println!(
            "Current physical memory usage: {} MB",
            usage.physical_mem / 1024 / 1024
        );
        println!(
            "Current virtual memory usage: {} MB",
            usage.virtual_mem / 1024 / 1024
        );
    } else {
        println!("Couldn't get the current memory usage :(");
    }
}
