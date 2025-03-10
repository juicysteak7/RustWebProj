use yew::prelude::*;
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;
// use serde::Deserialize;
// use serde::Serialize;
// use serde_json;
use crate::{ Application, ApplicationComponent };

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
                    <ApplicationComponent application={ Application{application_id: app.application_id.clone(), status: app.status.clone()} }/>
                }) }

                <ApplicationModal
                    is_open={self.is_modal_open}
                    on_close={link.callback(|_| Msg::CloseModal)}
                    on_submit={link.callback(|(id, status)| Msg::Add(Application{application_id:id, status}))}
                />
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ApplicationModalProps {
    pub on_close: Callback<()>,
    pub on_submit: Callback<(String, String)>,
    pub is_open: bool
}

pub struct ApplicationModal {
    application_id: String,
    status: String,
    is_open: bool,
}

pub enum ApplicationModalMsg {
    UpdateApplicationId(String),
    UpdateStatus(String),
    Submit,
    Close,
    Open,
}

impl Component for ApplicationModal {
    type Message = ApplicationModalMsg;
    type Properties = ApplicationModalProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self { application_id:"".to_string(), status:"".to_string(), is_open: ctx.props().is_open }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.is_open = ctx.props().is_open;
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ApplicationModalMsg::UpdateApplicationId(id) => {
                self.application_id = id;
                true
            }
            ApplicationModalMsg::UpdateStatus(status) => {
                self.status = status;
                true
            }
            ApplicationModalMsg::Submit => {
                ctx.props().on_submit.emit((self.application_id.clone(), self.status.clone()));
                ctx.props().on_close.emit(());
                true
            }
            ApplicationModalMsg::Close => {
                self.is_open = false;
                ctx.props().on_close.emit(());
                true
            }
            ApplicationModalMsg::Open => {
                self.is_open = true;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if !self.is_open {
            return html! {};
        }

        let link = ctx.link();
        html! {
            <div class="modal-overlay">
                <div class="modal-content">
                    <h2>{ "Add Application" }</h2>
                    <input
                        placeholder="Application ID"
                        value={self.application_id.clone()}
                        oninput={link.callback(|e: InputEvent| {
                            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                            ApplicationModalMsg::UpdateApplicationId(value)
                        })}
                    />
                    <input
                        placeholder="Status"
                        value={self.status.clone()}
                        oninput={link.callback(|e: InputEvent| {
                            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                            ApplicationModalMsg::UpdateStatus(value)
                        })}
                    />
                    <button onclick={link.callback(|_| ApplicationModalMsg::Submit)}>{ "Submit" }</button>
                    <button onclick={link.callback(|_| ApplicationModalMsg::Close)}>{ "Close" }</button>
                </div>
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