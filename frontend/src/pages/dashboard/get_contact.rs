use leptos::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Contact {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub mobile: String,
    pub organisation_name: String,
    pub message: String,
    pub created_at: String,
}

#[component]
pub fn GetContacts() -> impl IntoView {
    let (contacts, set_contacts) = create_signal(Vec::<Contact>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);

    spawn_local({
        let set_contacts = set_contacts.clone();
        let set_loading = set_loading.clone();
        let set_error = set_error.clone();

        async move {
            match Request::get("http://localhost:8000/contacts").send().await {
                Ok(resp) => {
                    if resp.ok() {
                        match resp.json::<Vec<Contact>>().await {
                            Ok(data) => set_contacts.set(data),
                            Err(err) => set_error.set(Some(format!("Failed to parse JSON: {}", err))),
                        }
                    } else {
                        set_error.set(Some(format!("HTTP error: {}", resp.status())));
                    }
                    set_loading.set(false);
                }
                Err(err) => {
                    set_error.set(Some(format!("Request failed: {}", err)));
                    set_loading.set(false);
                }
            }
        }
    });

    view! {
        <div class="contacts_container">
            <h2>"All Contacts"</h2>

            {move || {
                if loading.get() {
                    "Loading contacts...".into_view()
                } else if let Some(err) = error.get() {
                    view! { <p class="error">{err.clone()}</p> }.into_view()
                } else if contacts.get().is_empty() {
                    "No contacts found.".into_view()
                } else {
                    view! {
                        <table class="contacts_table">
                            <thead>
                                <tr>
                                    <th>"ID"</th>
                                    <th>"Name"</th>
                                    <th>"Email"</th>
                                    <th>"Mobile"</th>
                                    <th>"Organisation"</th>
                                    <th>"Message"</th>
                                    <th>"Created At"</th>
                                </tr>
                            </thead>
                            <tbody>
                                {contacts.get().iter().map(|c| view! {
                                    <tr key=c.id>
                                        <td>{c.id}</td>
                                        <td>{c.name.clone()}</td>
                                        <td>{c.email.clone()}</td>
                                        <td>{c.mobile.clone()}</td>
                                        <td>{c.organisation_name.clone()}</td>
                                        <td>{c.message.clone()}</td>
                                        <td>{c.created_at.clone()}</td>
                                    </tr>
                                }).collect::<Vec<_>>()}
                            </tbody>
                        </table>
                    }.into_view()
                }
            }}
        </div>
    }
}
