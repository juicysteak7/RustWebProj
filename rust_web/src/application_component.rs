use yew::prelude::*;
use serde_json;
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsCast;
use crate::TodoList;

use crate::{ UpdateApplicationModal, ApplicationData };

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub enum Status {
    InProgress,
    Applied,
    Rejected,
    Interviewing
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Applied => "Applied",
            Status::Rejected => "Rejected",
            Status::InProgress => "InProgress",
            Status::Interviewing => "Interviewing",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "Applied" => Status::Applied,
            "InProgress" => Status::InProgress,
            "Interviewing" => Status::Interviewing,
            "Rejected" => Status::Rejected,
            _ => Status::InProgress,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct Application {
    pub application_id: String,
    pub company: String,
    pub status: Status,
    pub job_title: String,
    pub location: String,
    pub link: String,
    pub application_date: String,
    pub tasks: Vec<String>,
}

pub enum Msg {
    Delete,
    Update,
    CloseModal,
    OpenModal,
    Updated(Application),
}

#[derive(Properties, PartialEq)]
pub struct ApplicationProps {
    pub application: Application,
    pub application_delete: Callback<Application>,
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

    fn update(&mut self, ctx: &Context<Self>, message: Self::Message) -> bool {
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
            },
            Msg::Updated(app) => {
                let old_id = self.application.application_id.clone();
                // self.application = Application { application_id: application_id.clone(), status:Status::from_str(status.as_str()), job_title: job_title.clone(), location: location.clone() };
                self.application = app.clone();
                spawn_local(async move {
                    match update_application(app, old_id).await {
                        Ok(result) => {
                            log::info!("Application updated: {:?}", result);
                        }
                        Err(e) => {
                            eprintln!("{}",e);
                        }
                    }
                });
                true
            },
            Msg::Delete => {
                let application = self.application.clone();
                spawn_local(async move {
                    match delete_application(application).await {
                        Ok(result) => {
                            log::info!("Application deleted: {:?}", result);
                        }
                        Err(e) => {
                            eprintln!("{}",e);
                        }
                    }
                });
                ctx.props().application_delete.emit(self.application.clone());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        // Disable background scroll and pointer events when modal is open
        if let Some(body) = web_sys::window().unwrap().document().unwrap().body() {
            let body: web_sys::Element = body.unchecked_into();
            let new_class = if self.is_modal_open { "modal-open" } else { "" };
            body.set_attribute("class", new_class).unwrap();
        }

        // Change class of application card when modal is open
        if let Some(element) = web_sys::window().unwrap().document().unwrap().get_element_by_id(&self.application.application_id) {
            let element: web_sys::Element = element.unchecked_into();
            let new_class = if self.is_modal_open { "application-card-modal-open" } else { "application-card" };
            element.set_attribute("class", new_class).unwrap();
        }

        html! {
            <div id={self.application.application_id.clone()} class="application-card">
                <p>{"Application Id: "}{self.application.application_id.clone()}</p>
                <p>{"Company: "}{self.application.company.clone()}</p>
                <p>{"Job Title: "}{self.application.job_title.clone()}</p>
                <p>{"Location: "}{self.application.location.clone()}</p>
                <p>{"Status: "}{self.application.status.as_str()}</p>
                <p>{"Date Applied: "}{self.application.application_date.clone()}</p>
                <a href={self.application.link.clone()} target="_blank">{"To Application"}</a>
                <br/>

                <UpdateApplicationModal
                is_open={self.is_modal_open}
                on_close={link.callback(|_| Msg::CloseModal)}
                on_submit={link.callback(|app| Msg::Updated(app))}
                application={self.application.clone()}
                />

                <TodoList application={self.application.clone()} tasks={self.application.tasks.clone()} on_update={link.callback(|app| Msg::Updated(app))} key={self.application.application_id.clone()}/>
                <button style="margin-top: 10px" onclick={link.callback(|_| Msg::OpenModal)}> {"Update"} </button>
                <button onclick={link.callback(|_| Msg::Delete)}> {"Delete"} </button>
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
async fn delete_application(application:Application) -> Result<ApplicationData, reqwest::Error> {
    let client = Client::new();
    let response = client.put("http://127.0.0.1:6969/api/delete_application").json(&application).send().await?;
    let data = response.json::<ApplicationResponse>().await?;
    Ok(data.applications)
}
