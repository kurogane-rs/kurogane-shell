#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use kurogane::App;

fn main() {
    App::new("{{frontend}}").run_or_exit();
}
