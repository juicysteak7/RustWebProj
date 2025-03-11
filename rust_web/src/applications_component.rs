use yew::prelude::*;
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;
// use serde::Deserialize;
// use serde::Serialize;
// use serde_json;
use crate::{ Application, ApplicationComponent, ApplicationModal };

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct ApplicationData {
    pub applications: Vec<Application>
}

pub enum Msg {
    Fetch,
    Fetched(ApplicationData),
    Add(Application),
    OpenModal,
    CloseModal,
}

#[derive(Properties, PartialEq, Debug)]
pub struct ApplicationsProps {
    pub applications: ApplicationData
}

#[derive(Debug)]
pub struct ApplicationsComponent {
    applications: ApplicationData,
    is_modal_open: bool,
}

impl Component for ApplicationsComponent {
    type Message = Msg;
    type Properties = ApplicationsProps;

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let props = ctx.props();
        self.applications = props.applications.clone();
        true
    }

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        Self { applications: props.applications.clone(), is_modal_open: false }
    }

    fn update(&mut self, ctx: &Context<Self>, message: Self::Message) -> bool{
        match message {
            Msg::Fetch => {
                let link = ctx.link().clone();
                spawn_local(async move {
                    match fetch_applications().await {
                        Ok(data) => {
                            log::info!("Applications fetched.");
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
                self.applications = data;
                true
            },
            Msg::Add(data) => {
                // let link = ctx.link().clone();

                self.applications.applications.push(data.clone());

                spawn_local(async move {
                    match add_application(data).await {
                        Ok(result) => {
                            log::info!("Application added: {:?}", result);
                        }
                        Err(e) => {
                            eprintln!("{}",e);
                        }
                    }
                });
                true
            },
            Msg::OpenModal => {
                self.is_modal_open = true;
                true
            }
            Msg::CloseModal => {
                self.is_modal_open = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html{
        let link = ctx.link();
        html! {
            <div>
                <h2>{"Applications"}</h2>
                <button onclick={link.callback(|_| Msg::Fetch)}>{ "Fetch Applications" }</button>
                <button onclick={link.callback(|_| Msg::OpenModal)}>{ "Add Application" }</button>
                { for self.applications.applications.iter().map(|app| html!{
                    <ApplicationComponent application={ Application{application_id: app.application_id.clone(), status: app.status.clone() } }/>
                }) }

                <ApplicationModal
                    is_open={self.is_modal_open}
                    on_close={link.callback(|_| Msg::CloseModal)}
                    on_submit={link.callback(|(id, status)| Msg::Add(Application{application_id: id, status }))}
                />
            </div>
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
struct ApplicationResponse {
    applications: ApplicationData
}
pub async fn fetch_applications() -> Result<ApplicationData, reqwest::Error> {
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