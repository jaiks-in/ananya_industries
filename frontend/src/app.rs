use leptos::*;
use leptos_router::*;
use crate::pages::homepage::homepage;
use crate::pages::dashboard::dashboard;
use crate::pages::auth::{auth,signup,login};
use crate::components::navbar;
pub fn App()->impl IntoView{
    view!{

       <Router>
            <navbar::NavBar/>
            <main class="app_container">
            <Routes >
            <Route path="/" view = homepage::HomePage/>
            <Route path="/dashboard" view= dashboard::Dashboard_Page/>
            <Route path="/login" view=login::LoginPage/>
            <Route path="/signup" view=signup::SignupPage/>
            </Routes>
            </main>
        </Router>

    }
}