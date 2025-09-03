use leptos::*;
use leptos_router::*;
use crate::components::NavBar;
use crate::pages::HomePage;
use crate::pages::Packaging_Box;
use crate::pages::contact_us;
pub fn App()->impl IntoView{
    view!{

        <Router>
         <NavBar::NavBar/>
        <main class="app_container">
            <Routes >
            <Route path="/" view = HomePage::HomePage/>
            <Route path="/packaging_box" view= Packaging_Box::Packaging_Box/>
            <Route path="/contact" view=contact_us::ContactUs/>
            </Routes>
        </main>
        </Router>

    }
}