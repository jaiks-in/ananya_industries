use leptos::*;
use gloo_net::http::Request;
use leptos_router::use_navigate;
use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, SubmitEvent};

#[component]
pub fn LoginPage() -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (error_message, set_error_message) = create_signal(String::new());
    let navigate = use_navigate();

    let login_handler = move |ev: SubmitEvent| {
        ev.prevent_default();
        let email_val = email.get_untracked();
        let password_val = password.get_untracked();
        let body = json!({
            "email": email_val,
            "password": password_val,
        });

        // We clone the `Maps` variable and use a new closure for `spawn_local`.
        // This ensures the outer `login_handler` can be called multiple times.
        let navigate = navigate.clone();
        spawn_local(async move {
            let request_result = Request::post("https://ananya-industries.onrender.com")
                .header("Content-Type", "application/json")
                .body(body.to_string());

            match request_result {
                Ok(req) => {
                    match req.send().await {
                        Ok(resp) => {
                            if resp.status() == 200 {
                                match resp.json::<serde_json::Value>().await {
                                    Ok(resp_json) => {
                                        if let Some(token) = resp_json["token"].as_str() {
                                            if let Some(storage) = web_sys::window()
                                                .and_then(|win| win.local_storage().ok().flatten())
                                            {
                                                if storage.set_item("token", token).is_err() {
                                                    set_error_message.set("Failed to save token to local storage.".to_string());
                                                } else {
                                                    navigate("/dashboard", Default::default());
                                                }
                                            } else {
                                                set_error_message.set("Local storage not available.".to_string());
                                            }
                                        } else {
                                            set_error_message.set("Invalid server response: token not found.".to_string());
                                        }
                                    },
                                    Err(_) => {
                                        set_error_message.set("Failed to parse server response.".to_string());
                                    }
                                }
                            } else {
                                // We've updated this message to include a sign-up link.
                                set_error_message.set("Invalid credentials. <a href=\"/signup\">Sign up here!</a>".to_string());
                            }
                        },
                        Err(e) => {
                            set_error_message.set(format!("Request failed: {:?}", e));
                        }
                    }
                },
                Err(e) => {
                    set_error_message.set(format!("Failed to build request: {:?}", e));
                }
            }
        });
    };

    view! {
        <div class="auth-container">
            <h1>"Login"</h1>
            // We now use `inner_html` to render the message with the clickable link.
            <p class="error-message" inner_html=move || error_message.get()></p>
            <form class="login-form" on:submit=login_handler>
                <label for="email">"Email"</label>
                <input
                    id="email"
                    type="email"
                    prop:value=email
                    on:input=move |ev| set_email.set(event_target_value(&ev))
                    required=true
                />

                <label for="password">"Password"</label>
                <input
                    id="password"
                    type="password"
                    prop:value=password
                    on:input=move |ev| set_password.set(event_target_value(&ev))
                    required=true
                />

                <button type="submit">"Login"</button>
            </form>
        </div>
    }
}
