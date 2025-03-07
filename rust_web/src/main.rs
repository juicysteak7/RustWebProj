use yew::prelude::*;
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MyData {
    message: String,
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| String::from(""));

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set("Getting data".to_string());

            let state = state.clone();
            // Query the backend on button click
            spawn_local(async move {
                match fetch_message().await {
                    Ok(data) => state.set(data.message),
                    Err(e) => state.set(e.to_string().into()),
                }
            })
        })
    };

    html! {
        <div>
            <button {onclick}>{ "Fetch Message" }</button>
            <p>{ (*state).clone() }</p>
        </div>
    }
}

async fn fetch_message() -> Result<MyData, reqwest::Error> {
    let client = Client::new();
    let response = client.get("http://127.0.0.1:6969/api/message").send().await?;
    let data = response.json::<MyData>().await?;
    Ok(data)
}

fn main() {
    yew::start_app::<App>();
}