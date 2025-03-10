use yew::prelude::*;
// use reqwest::Client;
// use wasm_bindgen_futures::spawn_local;
// use serde::Deserialize;
// use serde::Serialize;
use serde_json;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct Application {
    pub application_id: String,
    pub status: String,
}

#[derive(Properties, PartialEq)]
pub struct ApplicationProps {
    pub application: Application
}

pub struct ApplicationComponent {
    application: Option<Application>
}

impl Component for ApplicationComponent {
    type Message = ();
    type Properties = ApplicationProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        ApplicationComponent { application: Some(props.application.clone())  }
    }

    fn update(&mut self, _ctx: &Context<Self>, _message: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <p>{serde_json::to_string(&self.application).unwrap()}</p>
            </div>
        }
    }
}
