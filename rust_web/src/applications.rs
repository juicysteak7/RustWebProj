use yew::prelude::*;
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;
use serde::Deserialize;
use serde::Serialize;
use serde_json;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
struct ApplicationData {
    applications: Vec<Application>
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
struct Application {
    application_id: String,
    status: String,
}

pub enum Msg {
    Fetch,
    Fetched(ApplicationData),
    Add(Application)
}

pub struct ApplicationsComponent {
    applications: Option<ApplicationData>,
}

impl Component for ApplicationsComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { applications: None }
    }

    fn update(&mut self, ctx: &Context<Self>, message: Self::Message) -> bool{
        match message {
            Msg::Fetch => {
                let link = ctx.link().clone();
                spawn_local(async move {
                    match fetch_applications().await {
                        Ok(data) => {
                            link.send_message(Msg::Fetched(data));
                        }
                        Err(e) => {
                            eprintln!("{}",e);
                        }
                    }
                });
                false
            },
            Msg::Fetched(data) => {
                self.applications = Some(data);
                true
            },
            Msg::Add(data) => {
                let link = ctx.link().clone();
                spawn_local(async move {
                    match add_application(data).await {
                        Ok(data)=> {
                            link.send_message(Msg::Fetched(data));
                        }
                        Err(e) => {
                            eprintln!("{}",e);
                        }
                    }
                });
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html{
        let link = ctx.link();
        html! {
            <div>
                <h2>{"Applications"}</h2>
                <button onclick={link.callback(|_| Msg::Fetch)}>{ "Fetch Applications" }</button>
                <button onclick={link.callback(|_| Msg::Add(Application {application_id: "1".to_string(), status:"Applied".to_string()}))}>{ "Add Application" }</button>
                <p>{ self.applications.as_ref().map_or("No data yet.".to_string(), |data| serde_json::to_string(data).unwrap()) }</p>
            </div>
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
struct ApplicationResponse {
    applications: ApplicationData
}
async fn fetch_applications() -> Result<ApplicationData, reqwest::Error> {
    let client = Client::new();
    let response = client.get("http://127.0.0.1:6969/api/get_all_applications").send().await?;
    let data = response.json::<ApplicationResponse>().await?;
    Ok(data.applications)
}

async fn add_application(application:Application) -> Result<ApplicationData, reqwest::Error> {
    let client = Client::new();
    let response = client.put("http://127.0.0.1:6969/api/add_application").json(&application).send().await?;
    let data = response.json::<ApplicationResponse>().await?;
    Ok(data.applications)
}