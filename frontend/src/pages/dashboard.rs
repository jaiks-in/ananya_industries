use leptos::*;

#[component]
pub fn Dashboard_Page() -> impl IntoView {
    view! {
        <div id="dashboard-container">
            <aside id="sidebar">
                <h2>"Menu"</h2>
                <ul>
                    <li>"Users"</li>
                    <li>"Organisations"</li>
                    <li>"Products"</li>
                    <li>"Orders"</li>
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
            </main>
        </div>
    }
}
