#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use kurogane::App;

fn main() {
    #[cfg(debug_assertions)]
    {% if dev_url == "" %}App::new("{{frontend_dist}}").run_or_exit();{% else %}App::url("{{dev_url}}").run_or_exit();{% endif %}

    #[cfg(not(debug_assertions))]
    App::new("content").run_or_exit();
}
