use std::time::Duration;
use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let (current, set_current) = create_signal(0);
    let mut collection = vec![
        "https://res.cloudinary.com/dtc2dfweh/image/upload/v1756829697/Gemini_Generated_Image_6uaevh6uaevh6uae_p6uaqz.png",
        "https://res.cloudinary.com/dtc2dfweh/image/upload/v1756829697/Gemini_Generated_Image_rlx1eorlx1eorlx1_cxl3no.png",
        "https://res.cloudinary.com/dtc2dfweh/image/upload/v1756829694/Gemini_Generated_Image_2vt0vu2vt0vu2vt0_zxvgfm.png",
        "https://res.cloudinary.com/dtc2dfweh/image/upload/v1756829689/Gemini_Generated_Image_cbr682cbr682cbr6_yntnsz.png",
        "https://res.cloudinary.com/dtc2dfweh/image/upload/v1756829688/Gemini_Generated_Image_6511cs6511cs6511_dqedee.png",
        "https://res.cloudinary.com/dtc2dfweh/image/upload/v1756881022/ChatGPT_Image_Sep_3_2025_11_59_13_AM_bihzna.png"
    ];
    let total = collection.len();
    println!("{:?}", total);

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
            <section class="slider_section">
                <div class="slider">
                    <img src=move || collection[current.get() as usize] alt="slides"/>
                </div>
                <div class="slider_controls">
        <button class="prev_btn" on:click=move |_| {
                        let prev = if current.get() == 0 { total - 1 } else { current.get() - 1 };
                        set_current.set(prev);
                    }>"<<<"</button>
        <button class="next_btn" on:click=move |_| {
                        let next = if current.get() == total - 1 { 0 } else { current.get() + 1 };
                        set_current.set(next);
                    }>">>>"</button>

                </div>
            </section>

            <div class="about_section">
                <h2>About Ananya Industries</h2>
                <p>
                    Ananya Industries is a trusted name in the packaging industry, known for delivering
                    high-quality, durable, and innovative box solutions. We specialize in all types of boxes
                    from regular corrugated cartons to premium rigid boxes that enhance the look and feel
                    of any product.
                </p>



            </div>
        <div class="about_section">
        <div class="about_points">
                    <div class="point">
                        <h3 class="icon-star">Corrugated Boxes (Peti)</h3>
                        <p>
                            Strong, cost-effective, and reliable for bulk packaging, logistics, and transportation.
                        </p>
                    </div>

                    <div class="point">
                        <h3 class="icon-star">Duplex Boxes</h3>
                        <p>
                            Lightweight yet sturdy, ideal for FMCG, electronics, and retail packaging.
                        </p>
                    </div>

                    <div class="point">
                        <h3 class="icon-star">Rigid & Premium Boxes</h3>
                        <p>
                            Elegant and durable, perfect for luxury goods, gifts, jewelry, apparel,
                            and high-end branding.
                        </p>
                    </div>

                    <div class="point">
                        <h3 class="icon-star">Customized Packaging</h3>
                        <p>
                            Tailor-made designs, shapes, and sizes to suit the specific needs of every industry.
                        </p>
                    </div>
                </div>

        </div>
            </div>

    }
}
