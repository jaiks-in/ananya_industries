use leptos::*;
use crate::components::contact_form;
#[component]
pub fn ContactUs()->impl IntoView{
    view!{
        <div>
            <contact_form::ContactForm />
        </div>
    }
}