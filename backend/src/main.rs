use axum::{Router,routing::get};



#[tokio::main]
async fn main(){
    fn  add_user_details_handler(){

    }
let app=Router::new().route("/contact_us",post(add_user_details_handler));
}