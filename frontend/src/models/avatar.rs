// use leptos::*;
//
// #[component]
// pub fn Avatar(
//     cx: Scope,
//     /// Optional URL of the avatar image
//     avatar_url: Option<String>,
//     /// Full name of the user (for initials fallback)
//     full_name: String,
//     /// Size in pixels (optional)
//     size: Option<u32>,
// ) -> impl IntoView {
//     let size = size.unwrap_or(40);
//     let initials = full_name
//         .split_whitespace()
//         .map(|s| s.chars().next().unwrap_or('?'))
//         .take(2)
//         .collect::<String>()
//         .to_uppercase();
//
//     view! { cx,
//         <div
//             class="avatar"
//             style=format!("width:{}px; height:{}px; font-size:{}px;", size, size, size/2)
//         >
//             {
//                 if let Some(url) = avatar_url {
//                     view! { cx, <img src=url alt=full_name.clone()/> }
//                 } else {
//                     view! { cx, <span>{initials}</span> }
//                 }
//             }
//         </div>
//     }
// }
