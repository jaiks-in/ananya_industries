use leptos::*;
use web_sys::console;
#[component]
pub fn Contactform() -> impl IntoView {
    fn submit_handler(e){
       println!("{:?}",e);
    }
  view!{
        <div>
            <section class="feedback_section">
                <form on:sumbit=submit_handler>
                    <label for="name">Name</label>
                    <input id="name" type="text" placeholder="enter your name"/>
                </Form>
            </section>

      </div>
  }
}
