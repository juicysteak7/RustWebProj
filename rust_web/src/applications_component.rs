use yew::prelude::*;
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;
// use serde::Deserialize;
// use serde::Serialize;
// use serde_json;
use crate::{ Application, ApplicationComponent, AddApplicationModal, Status };
use wasm_bindgen::JsCast;

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
    DeleteApplication(Application),
}

#[derive(Properties, PartialEq, Debug)]
pub struct ApplicationsProps {
    pub applications: Vec<Application>
}

#[derive(Debug)]
pub struct ApplicationsComponent {
    applications: Vec<Application>,
    len: usize,
    is_modal_open: bool,
}

impl Component for ApplicationsComponent {
    type Message = Msg;
    type Properties = ApplicationsProps;

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let props = ctx.props();
        self.applications = props.applications.clone();
        self.len = self.applications.len();
        true
    }

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        Self { applications: props.applications.clone(), len: props.applications.len(), is_modal_open: false }
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
                true
            },
            Msg::Fetched(data) => {
                log::info!("apps: {:?}", data.applications.clone());
                self.applications = data.applications.clone();
                self.len = data.applications.len();
                true
            },
            Msg::Add(data) => {
                // let link = ctx.link().clone();

                self.applications.push(data.clone());
                self.len = self.applications.len();

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
            },
            Msg::CloseModal => {
                self.is_modal_open = false;
                true
            },
            Msg::DeleteApplication(app) => {
                self.applications.retain(|application| application.application_id != app.application_id);
                self.len = self.applications.len();
                log::info!("remaining apps: {:?}", self.applications);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html{
        let link = ctx.link();
        // Disable background scroll and pointer events when modal is open
        if let Some(body) = web_sys::window().unwrap().document().unwrap().body() {
            let body: web_sys::Element = body.unchecked_into();
            let new_class = if self.is_modal_open { "modal-open" } else { "" };
            body.set_attribute("class", new_class).unwrap();
        }
        html! {
            <div class="applications-container">
                <h2>{"Applications"}</h2>
                <button onclick={link.callback(|_| Msg::Fetch)}>{ "Fetch Applications" }</button>
                <button onclick={link.callback(|_| Msg::OpenModal)}>{ "Add Application" }</button>
                <AddApplicationModal
                    is_open={self.is_modal_open}
                    on_close={link.callback(|_| Msg::CloseModal)}
                    on_submit={link.callback(|app| Msg::Add(app))}
                    application_id={self.len}
                />

                { for self.applications.iter().map(|app| html!{
                    <div class="applications-list" key={app.application_id.clone()}>
                        <ApplicationComponent key={app.application_id.clone()}
                            application={ app.clone() } 
                            application_delete={link.callback(|app| Msg::DeleteApplication(app))}
                        />
                    </div>
                }) }
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