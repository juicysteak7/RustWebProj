use yew::prelude::*;
use serde_json;
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;

use crate::{ UpdateApplicationModal, ApplicationData };

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct Application {
    pub application_id: String,
    pub status: String,
}

pub enum Msg {
    Update,
    CloseModal,
    OpenModal,
    Updated(String, String),
}

#[derive(Properties, PartialEq)]
pub struct ApplicationProps {
    pub application: Application
}

pub struct ApplicationComponent {
    application: Application,
    is_modal_open: bool
}

impl Component for ApplicationComponent {
    type Message = Msg;
    type Properties = ApplicationProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        ApplicationComponent { application: props.application.clone(), is_modal_open: false  }
    }

    fn update(&mut self, _ctx: &Context<Self>, message: Self::Message) -> bool {
        match message {
            Msg::Update => {
                log::info!("Update Button Pressed.");
                false
            },
            Msg::CloseModal => {
                self.is_modal_open = false;
                true
            },
            Msg::OpenModal => {
                self.is_modal_open = true;
                true
            }
            Msg::Updated(application_id, status) => {
                let old_id = self.application.application_id.clone();
                self.application = Application { application_id: application_id.clone(), status:status.clone() };
                spawn_local(async move {
                    match update_application(Application { application_id, status }, old_id).await {
                        Ok(result) => {
                            log::info!("Application updated: {:?}", result);
                        }
                        Err(e) => {
                            eprintln!("{}",e);
                        }
                    }
                });
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
                <p>{serde_json::to_string(&self.application).unwrap()}</p>
                <button onclick={link.callback(|_| Msg::OpenModal)}> {"Update"} </button>

                <UpdateApplicationModal
                is_open={self.is_modal_open}
                on_close={link.callback(|_| Msg::CloseModal)}
                on_submit={link.callback(|(id, status)| Msg::Updated(id, status))}
                application_id={self.application.application_id.clone()}
                status={self.application.status.clone()}
                />
            </div>
        }
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
struct ApplicationResponse {
    applications: ApplicationData
}
async fn update_application(application:Application, old_id:String) -> Result<ApplicationData, reqwest::Error> {
    let client = Client::new();
    let response = client.put("http://127.0.0.1:6969/api/update_application").json(&(application,old_id)).send().await?;
    let data = response.json::<ApplicationResponse>().await?;
    Ok(data.applications)
}
