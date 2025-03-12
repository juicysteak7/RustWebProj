use yew::prelude::*;

use crate::{ Status, Application };

#[derive(Properties, PartialEq)]
pub struct ApplicationModalProps {
    pub on_close: Callback<()>,
    pub on_submit: Callback<Application>,
    pub is_open: bool
}

pub struct AddApplicationModal {
    application: Application,
    is_open: bool,
}

pub enum Msg {
    UpdateApplicationId(String),
    UpdateStatus(String),
    UpdateTitle(String),
    UpdateLocation(String),
    Submit,
    Close,
    Open,
}

impl Component for AddApplicationModal {
    type Message = Msg;
    type Properties = ApplicationModalProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self { application: Application {application_id: "".to_string(), status: Status::from_str("Pending"), job_title: "".to_string(), location: "".to_string()}, is_open: ctx.props().is_open.clone() }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.is_open = ctx.props().is_open;
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateApplicationId(id) => {
                self.application.application_id = id;
                true
            }
            Msg::UpdateStatus(status) => {
                self.application.status = Status::from_str(&status);
                true
            }
            Msg::UpdateTitle(job_title) => {
                self.application.job_title = job_title;
                true
            }
            Msg::UpdateLocation(location) => {
                self.application.location = location;
                true
            }
            Msg::Submit => {
                ctx.props().on_submit.emit(self.application.clone());
                ctx.props().on_close.emit(());
                self.is_open = false;
                true
            }
            Msg::Close => {
                ctx.props().on_close.emit(());
                self.is_open = false;
                true
            }
            Msg::Open => {
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
            <div class="modal">
                <div class="modal-content">
                    <h2>{ "Add Application" }</h2>
                    <form class="modal-form">
                        <input
                            placeholder="Application ID"
                            value={self.application.application_id.clone()}
                            oninput={link.callback(|e: InputEvent| {
                                let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                Msg::UpdateApplicationId(value)
                            })}
                        />
                        <br/>
                        <input
                            placeholder="Job Title"
                            value={self.application.job_title.clone()}
                            oninput={link.callback(|e: InputEvent| {
                                let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                Msg::UpdateTitle(value)
                            })}
                        />
                        <br/>
                        <input
                            placeholder="Location"
                            value={self.application.location.clone()}
                            oninput={link.callback(|e: InputEvent| {
                                let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                Msg::UpdateLocation(value)
                            })}
                        />
                        <br/>
                        <select
                            onchange={link.callback(|e: web_sys::Event| {
                                let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                Msg::UpdateStatus(value)
                            })}
                        >
                        {
                            for vec!["Applied", "InProgress", "Interviewing", "Rejected"].iter().map(|status| html! {
                                <option 
                                    value={status.to_string()}
                                    selected={self.application.status.as_str() == *status}>
                                    { status }
                                </option>
                            })
                        }
                        </select>
                        <br/>
                    </form>
                    <div class="modal-actions">
                        <button onclick={link.callback(|_| Msg::Submit)}>{ "Submit" }</button>
                        <button onclick={link.callback(|_| Msg::Close)}>{ "Close" }</button>
                    </div>
                </div>
            </div>
        } 
    }
}