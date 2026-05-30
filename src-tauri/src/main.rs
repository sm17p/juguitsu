#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    color_eyre::install().expect("failed to install color-eyre");
    juguitsu_lib::run()
}
