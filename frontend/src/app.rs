use leptos::*;
use leptos_router::*;
use crate::components::navbar;
use crate::pages::homepage;
use crate::pages::packaging_box;
use crate::pages::contact_us;
use crate::pages::dashboard;
use crate::pages::auth::auth;
use crate::pages::auth::signup;
pub fn App()->impl IntoView{
    view!{

        <Router>
         <navbar::NavBar/>
        <main class="app_container">
            <Routes >
            <Route path="/" view = homepage::HomePage/>
            <Route path="/packaging_box" view= packaging_box::Packaging_Box/>
            <Route path="/dashboard" view= dashboard::Dashboard_Page/>
            <Route path="/contact" view=contact_us::ContactUs/>
            <Route path="/login" view=auth::AuthPage/>
            <Route path="/signup" view=signup::SignupPage/>
            </Routes>
        </main>
        </Router>

    }
}