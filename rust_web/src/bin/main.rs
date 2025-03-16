use yew::prelude::*;
use rust_web::{ ApplicationsComponent, fetch_applications, ApplicationData };
use wasm_bindgen_futures::spawn_local;

#[function_component(App)]
fn app() -> Html {
    let init = use_state(|| ApplicationData { applications: vec![] });

    {
        let init = init.clone();
        use_effect_with_deps(move |_| {
            spawn_local(async move {
                match fetch_applications().await {
                    Ok(data) => {
                        init.set(data);
                    }
                    Err(e) => {
                        eprintln!("{}",e);
                    }
                };
            });
            || ()
        }, ());

    }

    html! {
        <div>
            <ApplicationsComponent applications={ (*init).applications.clone() }/>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}