use std::time::Duration;
use leptos::*;
use crate::pages::homepage::contact_form;

#[component]
pub fn HomePage() -> impl IntoView {
    let (current, set_current) = create_signal(0);
    let (open_contact_form,set_open_contact_form)=create_signal(false);
    let collection = vec![
        "https://res.cloudinary.com/dtc2dfweh/image/upload/v1756829697/Gemini_Generated_Image_6uaevh6uaevh6uae_p6uaqz.png",
        "https://res.cloudinary.com/dtc2dfweh/image/upload/v1756829697/Gemini_Generated_Image_rlx1eorlx1eorlx1_cxl3no.png",

    ];
    let total = collection.len();

    leptos::set_interval(
        move || {
            set_current.update(move |i| {
                *i = if *i == total - 1 { 0 } else { *i + 1 }
            })
        },
        Duration::from_secs(2),
    );
    view! {
        <div class="homepage_container">
            <div class="left_column">
                <div class="slider_section">

                    <div class="slider">
        <div class="slider_text">
            <h1>"Welcome to Ananya Industries"</h1>
        </div>
                        <img src=move || collection[current.get()] alt="slides"/>
                    <div class="homepage_btn_container">
                    <button class="contact_us_btn" on:click=move|_|set_open_contact_form.set(true) >
                    Contact Us
                    </button>
                    </div>
                    </div>
                </div>
{move || {
                if open_contact_form.get() {
                    view! {
                        <div class="modal_overlay">
                            <div class="modal_content">
                                <contact_form::ContactForm />
                                <button
                                    class="close-btn"
                                    on:click=move |_| set_open_contact_form.set(false)>
                                    "Close"
                                </button>
                            </div>
                        </div>
                    }
                } else {
                    view! { <div></div> }
                }
            }}
                <div class="about_points">
                    <div class="point">
                        <h3>"Corrugated Boxes (Peti)"</h3>
                        <p>
                            "Strong, cost-effective, and reliable for bulk packaging, logistics, and transportation."
                        </p>
                    </div>

                    <div class="point">
                        <h3>"Duplex Boxes"</h3>
                        <p>
                            "Lightweight yet sturdy, ideal for FMCG, electronics, and retail packaging."
                        </p>
                    </div>

                    <div class="point">
                        <h3>"Rigid & Premium Boxes"</h3>
                        <p>
                            "Elegant and durable, perfect for luxury goods, gifts, jewelry, apparel,
                            and high-end branding."
                        </p>
                    </div>

                    <div class="point">
                        <h3>"Customized Packaging"</h3>
                        <p>
                            "Tailor-made designs, shapes, and sizes to suit the specific needs of every industry."
                        </p>
                    </div>
                </div>
            </div>

            <div class="right_column">
                <div class="about_section">
                    <h2>"About Ananya Industries"</h2>
                    <p>
                        "Ananya Industries is a trusted name in the packaging industry, known for delivering
                        high-quality, durable, and innovative box solutions. We specialize in all types of boxes
                        from regular corrugated cartons to premium rigid boxes that enhance the look and feel
                        of any product."
                    </p>
                </div>

            </div>
        </div>
    }
}
