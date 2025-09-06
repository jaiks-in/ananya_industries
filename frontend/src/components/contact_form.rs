use leptos::*;
use web_sys::SubmitEvent;
use gloo_net::http::Request;
use serde_json::json;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn ContactForm() -> impl IntoView {
    let (full_name,set_full_name)=create_signal(String::new());
    let (organisation_name,set_organisation_name)=create_signal(String::new());
    let (mobile_number, set_mobile_number) = create_signal("".to_string());
    let (email, set_email) = create_signal("".to_string());
    let (message, set_message) = create_signal("".to_string());
    let (submission, set_submission) = create_signal("".to_string());
    let (modal, set_modal) = create_signal(false);

    // Submit handler
    let submit_handler = move |ev: SubmitEvent| {
        ev.prevent_default();
        // Collect values
        let body = json!({
            "name": full_name,
            "email": email.get(),
            "organisation_name": organisation_name.get(), // <-- यहाँ परिवर्तन किया गया है
            "mobile": mobile_number.get(),
            "message": message.get(),
        });
        logging::log!("{}",body.to_string());
        spawn_local(async move {
            let request = Request::post("https://ananya-industries.onrender.com/contact")
                .header("Content-Type", "application/json")
                .body(body.to_string());

            match request {
                Ok(req) => {
                    match req.send().await {
                        Ok(resp) => {
                            if resp.ok() {
                                set_submission.set("✅ Inquiry submitted successfully!".to_string());
                            } else {
                                set_submission.set(format!("❌ Server error: {}", resp.status()));
                            }
                        }
                        Err(err) => {
                            set_submission.set(format!("❌ Request failed: {err:?}"));
                        }
                    }
                }
                Err(err) => {
                    set_submission.set(format!("❌ Could not build request: {err:?}"));
                }
            }

            set_modal.set(true);

            // Reset fields
            set_full_name.set("".to_string());
            set_organisation_name.set("".to_string());
            set_mobile_number.set("".to_string());
            set_email.set("".to_string());
            set_message.set("".to_string());
        });

    };

    view! {
        <div>
            <section class="contact_section">
                <form class="inquiry_form" on:submit=submit_handler>
                    <label for="name">"Enter your Full Name"</label>
                    <input id="name" type="text"
                        value=full_name.get()
                        on:input=move |ev| set_full_name.set(event_target_value(&ev))
                        required=true
                    />

                    <label for="organisation-name">"Enter your Organisation Name"</label>
                    <input id="organisation-name" type="text"
                        value=organisation_name.get()
                        on:input=move |ev| set_organisation_name.set(event_target_value(&ev))
                        required=true
                    />

                    <label for="mobile">"Mobile Number"</label>
                    <input id="mobile" type="text"
                        value=mobile_number.get()
                        on:input=move |ev| set_mobile_number.set(event_target_value(&ev))
                    />

                    <label for="email">"Email"</label>
                    <input id="email" type="email"
                        value=email.get()
                        on:input=move |ev| set_email.set(event_target_value(&ev))
                    />

                    <label for="inquiry">"Inquiry"</label>
                    <textarea id="inquiry" rows=4
                        value=message.get()
                        on:input=move |ev| set_message.set(event_target_value(&ev))
                    ></textarea>

                    <button type="submit">"Submit"</button>
                </form>
            </section>

            {move || if modal.get() {
                view! {
                    <div class="modal_overlay">
                        <div class="modal_content">
                            <h2>"Thank you!"</h2>
                            <p>{submission.get()}</p>
                            <button on:click=move |_| set_modal.set(false)>"Close"</button>
                        </div>
                    </div>
                }
            } else {
                view! { <div></div> }
            }}
        </div>
    }
}
