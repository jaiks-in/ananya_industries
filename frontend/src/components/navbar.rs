use leptos::*;
use leptos_router::*;

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <header class="navbar">
            <div class="logo">
                <img src="https://res.cloudinary.com/dtc2dfweh/image/upload/v1756829695/Gemini_Generated_Image_wgtt2dwgtt2dwgtt_fxtwmx.png"  alt="Sterling Logo"/>
            </div>

            <nav class="navbar_item">
                <A href="/">"Home"</A>
                <A href="/dashboard">"Dashboard"</A>
                <A href="/login">"Login"</A>
            </nav>
        </header>
    }
}
