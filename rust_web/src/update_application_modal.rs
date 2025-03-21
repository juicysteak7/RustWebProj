use yew::prelude::*;

use crate::{ Status, Application };

pub enum Msg {
    Submit,
    Close,
    UpdateStatus(String),
    // UpdateApplicationId(String),
    UpdateTitle(String),
    UpdateLocation(String),
    UpdateLink(String),
    UpdateCompany(String),
    UpdateDate(String),
}

#[derive(Properties, PartialEq)]
pub struct UpdateProps {
    pub on_close: Callback<()>,
    pub on_submit: Callback<Application>,
    pub application: Application,
    pub is_open: bool,
}
pub struct UpdateApplicationModal {
    application: Application,
    is_open: bool
}

impl Component for UpdateApplicationModal {
    type Message = Msg;
    type Properties = UpdateProps;

    fn create(ctx: &Context<Self>) -> Self{
        Self { application: ctx.props().application.clone(), is_open: ctx.props().is_open.clone() }
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
                ctx.props().on_submit.emit(self.application.clone());
                ctx.props().on_close.emit(());
                self.is_open = false;
                true
            }
            // Msg::UpdateApplicationId(application_id) => {
            //     self.application.application_id = application_id;
            //     true
            // }
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
            Msg::UpdateLink(link) => {
                self.application.link = link;
                true
            } 
            Msg::UpdateCompany(company)=> {
                self.application.company = company;
                true
            }
            Msg::UpdateDate(date) => {
                self.application.application_date = date;
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
            <div class="sub-modal">
                <div class="modal-content">
                    <h2>{ "Update Application" }</h2>
                    <form class="modal-form">
                    <div class="input-container">
                        <input
                            placeholder="Company"
                            value={self.application.company.clone()}
                            oninput={link.callback(|e: InputEvent| {
                                let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                Msg::UpdateCompany(value)
                            })}
                        />
                    </div>
                    <div class="input-container">
                        <input
                            placeholder={self.application.job_title.clone()}
                            value={self.application.job_title.clone()}
                            oninput={link.callback(|e: InputEvent| {
                                let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                Msg::UpdateTitle(value)
                            })}
                        />
                    </div>
                    <div class="input-container">
                        <input
                            placeholder={self.application.location.clone()}
                            value={self.application.location.clone()}
                            oninput={link.callback(|e: InputEvent| {
                                let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                Msg::UpdateLocation(value)
                            })}
                        />
                    </div>
                    <div class="input-container">
                        <input
                            placeholder={self.application.application_date.clone()}
                            value={self.application.application_date.clone()}
                            oninput={link.callback(|e: InputEvent| {
                                let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                Msg::UpdateDate(value)
                            })}
                        />
                    </div>
                    <div class="input-container">
                        <input
                            placeholder={self.application.link.clone()}
                            value={self.application.link.clone()}
                            oninput={link.callback(|e: InputEvent| {
                                let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                Msg::UpdateLink(value)
                            })}
                        />
                    </div>
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