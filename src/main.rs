use leptos::*;
use leptos_wasm_github_pages::App;

fn main() {
    // стандартное логирование, но вдруг кому пригодится
    // _ = console_log::init_with_level(log::Level::Debug);
    // console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <App />
        }
    })
}
