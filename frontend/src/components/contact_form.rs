use leptos::*;
use web_sys::SubmitEvent;

#[component]
pub fn ContactForm() -> impl IntoView {
    let (name, set_name) = create_signal("".to_string());
    let (middlename, set_middlename) = create_signal("".to_string());
    let (lastname, set_lastname) = create_signal("".to_string());
    let (mobile_number, set_mobile_number) = create_signal("".to_string());
    let (email, set_email) = create_signal("".to_string());
    let (inquiry, set_inquiry) = create_signal("".to_string());
    let (submission, set_submission) = create_signal("".to_string());
    let (modal, set_modal) = create_signal(false);

    // Submit handler (closure so it captures signals)
    let submit_handler = move |ev: SubmitEvent| {
        ev.prevent_default();

        let full_name = format!(
            "{} {} {}",
            name.get(),
            middlename.get(),
            lastname.get()
        );

        let details = format!(
            "Name: {}\nMobile: {}\nEmail: {}\nInquiry: {}",
            full_name,
            mobile_number.get(),
            email.get(),
            inquiry.get()
        );

        set_submission.set(details); // store submission
        set_modal.set(true);         // show modal

        // Reset form fields
        set_name.set("".to_string());
        set_middlename.set("".to_string());
        set_lastname.set("".to_string());
        set_mobile_number.set("".to_string());
        set_email.set("".to_string());
        set_inquiry.set("".to_string());
    };

    view! {
        <div>
            <section class="contact_section">
                <form class="inquiry_form" on:submit=submit_handler>
                    <label for="name">"Enter your Name"</label>
                    <input
                        type="text"
                        id="name"
                        value=name.get()
                        on:input=move |ev| set_name.set(event_target_value(&ev))
                        required=true
                    />

                    <label for="middlename">"Middle Name"</label>
                    <input
                        type="text"
                        id="middlename"
                        value=middlename.get()
                        on:input=move |ev| set_middlename.set(event_target_value(&ev))
                    />

                    <label for="lastname">"Last Name"</label>
                    <input
                        type="text"
                        id="lastname"
                        value=lastname.get()
                        on:input=move |ev| set_lastname.set(event_target_value(&ev))
                    />

                    <label for="mobile">"Mobile Number"</label>
                    <input
                        type="text"
                        id="mobile"
                        value=mobile_number.get()
                        on:input=move |ev| set_mobile_number.set(event_target_value(&ev))
                    />

                    <label for="email">"Email"</label>
                    <input
                        type="email"
                        id="email"
                        value=email.get()
                        on:input=move |ev| set_email.set(event_target_value(&ev))
                    />

                    <label for="inquiry">"Inquiry"</label>
                    <textarea
                        id="inquiry"
                        rows=4
                        value=inquiry.get()
                        on:input=move |ev| set_inquiry.set(event_target_value(&ev))
                    ></textarea>

                    <button type="submit">"Submit"</button>
                </form>
            </section>

            // Modal
            {move || if modal.get() {
                view! {
                    <div class="modal_overlay">
                        <div class="modal_content">
                            <h2>"Thank you!"</h2>
                            <p>"Your inquiry has been submitted successfully."</p>
                            <pre>{submission.get()}</pre>
                            <button on:click=move |_| set_modal.set(false)>"Close"</button>
                        </div>
                    </div>
                }
            } else {
                view! {<div></div>}
            }}
        </div>
    }
}
