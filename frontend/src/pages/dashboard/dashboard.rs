use leptos::*;
use crate::pages::dashboard::get_contact;
#[component]
pub fn Dashboard_Page() -> impl IntoView {
    let (open_contacts,set_open_contacts)=create_signal(false);
    view! {
        <div id="dashboard-container">
            <aside id="sidebar">
                <h2>"Menu"</h2>
                <ul>
                    <li>"Users"</li>
                    <li>"Organisations"</li>
                    <li>"Products"</li>
                    <li>"Orders"</li>
                    <li on:click=move|_|{ set_open_contacts.set(true) }>
                    "Contacted Users"</li>
                </ul>
            </aside>

            <main id="main-content">
                <h1>"Analytics & Orders"</h1>
                <div class="graph-placeholder">
                    "Graph / Charts will appear here"
                </div>
                <div class="stats-placeholder">
                    "Other stats / analytics"
                </div>
                <div>
                        <get_contact::GetContacts />
                </div>
            </main>
        </div>
    }
}
