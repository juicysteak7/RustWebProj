use yew::prelude::*;
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MyData {
    message: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
struct Application {
    application_id: String,
    status: String,
}

#[function_component(App)]
fn app() -> Html {
    let applications = use_state(|| String::from(""));

    let onclick_add = {
        let applications = applications.clone();
        Callback::from(move |_| {
            let applications = applications.clone();
            // Query the backend on button click
            spawn_local(async move {
                match add_application(Application {application_id:"1".to_string(), status: "Applied".to_string()}).await {
                    Ok(data) => applications.set(data.message),
                    Err(e) => applications.set(e.to_string().into()),
                }
            })
        })
    };

    let onclick_fetch = {
        let applications = applications.clone();
        Callback::from(move |_| {
            let applications = applications.clone();
            // Query the backend on button click
            spawn_local(async move {
                match fetch_applications().await {
                    Ok(data) => applications.set(data.message),
                    Err(e) => eprint!("Error: {}", e),
                }
            })
        })
    };

    html! {
        <div>
            <div>
                <h2>{"Applications"}</h2>
                <button onclick={onclick_fetch}>{ "Fetch Applications" }</button>
                <button onclick={onclick_add}>{ "Add Application" }</button>
                <p>{(*applications).clone()}</p>
            </div>
        </div>
    }
}

async fn fetch_applications() -> Result<MyData, reqwest::Error> {
    let client = Client::new();
    let response = client.get("http://127.0.0.1:6969/api/get_all_applications").send().await?.text().await?;
    //let data = response.json::<Vec<Application>>().await?;
    Ok(MyData {
        message: response
    })
}

async fn add_application(application:Application) -> Result<MyData, reqwest::Error> {
    let client = Client::new();
    let response = client.put("http://127.0.0.1:6969/api/add_application").json(&application).send().await?;
    let data = response.json::<MyData>().await?;
    Ok(data)
}

fn main() {
    yew::start_app::<App>();
}