use yew::prelude::*;
use rust_web::{ ApplicationsComponent };

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <ApplicationsComponent />
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}