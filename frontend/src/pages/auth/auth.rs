use leptos::*;
use leptos::ev::SubmitEvent;
use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use crate::pages::auth::login;
#[component]
pub fn AuthPage() -> impl IntoView {

view!{
    <login::LoginPage/>
}

}
