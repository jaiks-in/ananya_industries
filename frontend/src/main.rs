

use leptos::*;
mod app;
mod components;
mod pages;
mod models;

fn main() {
    // Mount Leptos app into <body>
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <app::App/> })
}
