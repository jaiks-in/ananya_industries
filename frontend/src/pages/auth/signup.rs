use leptos::*;
use gloo_net::http::Request;
use leptos_router::use_navigate;
use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use web_sys::SubmitEvent;
use leptos_router::A;

#[component]
pub fn SignupPage() -> impl IntoView {
    let (username, set_username) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (confirm_password, set_confirm_password) = create_signal(String::new());
    let (organisation_name, set_organisation_name) = create_signal(String::new());
    let (error_message, set_error_message) = create_signal(String::new());

    let navigate = use_navigate();
    let navigate_login=use_navigate();

    let signup_handler = move |ev: SubmitEvent| {
        ev.prevent_default();

        let username_val = username.get_untracked();
        let email_val = email.get_untracked();
        let password_val = password.get_untracked();
        let confirm_password_val = confirm_password.get_untracked();
        let organisation_name_val = organisation_name.get_untracked();

        if password_val != confirm_password_val {
            set_error_message.set("Passwords do not match.".to_string());
            return;
        }

        let body = json!({
            "username": username_val,
            "email": email_val,
            "password": password_val,
            "organisation_name": organisation_name_val,
        });

        let navigate = navigate.clone();
        let set_error_message = set_error_message.clone();

        spawn_local(async move {
            let request_result = Request::post("https://ananya-industries.onrender.com/signup")
                .header("Content-Type", "application/json")
                .body(body.to_string());

            match request_result {
                Ok(req) => match req.send().await {
                    Ok(resp) => {
                        if resp.status() == 201 {
                            navigate("/login", Default::default());
                        } else {
                            set_error_message.set(format!("Signup failed: {}", resp.status()));
                        }
                    }
                    Err(e) => {
                        set_error_message.set(format!("Request failed: {:?}", e));
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
            <h1>"Sign Up"</h1>
            <p class="error-message">{move || error_message.get()}</p>
            <form class="signup-form" on:submit=signup_handler>
                <label for="username">"Username"</label>
                <input
                    id="username"
                    type="text"
                    prop:value=username
                    on:input=move |ev| set_username.set(event_target_value(&ev))
                    required=true
                />

                <label for="email">"Email"</label>
                <input
                    id="email"
                    type="email"
                    prop:value=email
                    on:input=move |ev| set_email.set(event_target_value(&ev))
                    required=true
                />

                <label for="organisation-name">"Organisation Name"</label>
                <input
                    id="organisation-name"
                    type="text"
                    prop:value=organisation_name
                    on:input=move |ev| set_organisation_name.set(event_target_value(&ev))
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

                <label for="confirm-password">"Confirm Password"</label>
                <input
                    id="confirm-password"
                    type="password"
                    prop:value=confirm_password
                    on:input=move |ev| set_confirm_password.set(event_target_value(&ev))
                    required=true
                />

                <button type="submit">"Sign Up"</button>
            </form>

            <p class="login_suggestion">
    "Already have an account? "
    <span style="color: blue; cursor: pointer;" on:click=move |_| {
        navigate_login("/login", Default::default());
    }>
        "Log in here!"
    </span>
</p>
        </div>
    }
}
