use yew::prelude::*;

use crate::{ Status };

pub enum Msg {
    Submit,
    Close,
    UpdateStatus(String),
    UpdateApplicationId(String),
}

#[derive(Properties, PartialEq)]
pub struct UpdateProps {
    pub on_close: Callback<()>,
    pub on_submit: Callback<(String, Status)>,
    pub application_id: String,
    pub status: Status,
    pub is_open: bool,
}
pub struct UpdateApplicationModal {
    application_id: String,
    status: String,
    is_open: bool
}

impl Component for UpdateApplicationModal {
    type Message = Msg;
    type Properties = UpdateProps;

    fn create(ctx: &Context<Self>) -> Self{
        Self { application_id: ctx.props().application_id.clone() , status: ctx.props().status.as_str().to_string() , is_open: ctx.props().is_open }
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.is_open = ctx.props().is_open;
        true
    }
    fn update(&mut self, ctx: &Context<Self>, message: Self::Message) -> bool {
        match message {
            Msg::Close => {
                ctx.props().on_close.emit(());
                self.is_open = false;
                false
            }
            Msg::Submit => {
                ctx.props().on_submit.emit((self.application_id.clone(), Status::from_str(self.status.as_str())));
                ctx.props().on_close.emit(());
                self.is_open = false;
                true
            }
            Msg::UpdateApplicationId(application_id) => {
                self.application_id = application_id;
                true
            }
            Msg::UpdateStatus(status) => {
                self.status = status;
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        if !self.is_open {
            return html! {
                <div></div>
            }
        }
        let link = ctx.link();
        html! {
            <div>
                <h2>{ "Update Application" }</h2>
                <input
                    placeholder={self.application_id.clone()}
                    value={self.application_id.clone()}
                    oninput={link.callback(|e: InputEvent| {
                        let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                        Msg::UpdateApplicationId(value)
                    })}
                />
                <input
                    placeholder={self.status.clone()}
                    value={self.status.clone()}
                    oninput={link.callback(|e: InputEvent| {
                        let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                        Msg::UpdateStatus(value)
                    })}
                />
                <button onclick={link.callback(|_| Msg::Submit)}>{ "Submit" }</button>
                <button onclick={link.callback(|_| Msg::Close)}>{ "Close" }</button>
            </div>
        }
    }
}