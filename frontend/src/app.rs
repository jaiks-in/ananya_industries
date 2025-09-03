use leptos::*;
use leptos_router::*;
use crate::components::navbar;
use crate::pages::homepage;
use crate::pages::packaging_box;
use crate::pages::contact_us;
pub fn App()->impl IntoView{
    view!{

        <Router>
         <navbar::NavBar/>
        <main class="app_container">
            <Routes >
            <Route path="/" view = homepage::HomePage/>
            <Route path="/packaging_box" view= packaging_box::Packaging_Box/>
            <Route path="/contact" view=contact_us::ContactUs/>
            </Routes>
        </main>
        </Router>

    }
}