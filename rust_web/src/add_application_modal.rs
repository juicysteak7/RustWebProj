use yew::prelude::*;

use crate::{ Status, Application };

#[derive(Properties, PartialEq)]
pub struct ApplicationModalProps {
    pub on_close: Callback<()>,
    pub on_submit: Callback<Application>,
    pub is_open: bool,
    pub application_id: usize,
}

pub struct AddApplicationModal {
    application: Application,
    is_open: bool,
}

pub enum Msg {
    // UpdateApplicationId(String),
    UpdateStatus(String),
    UpdateTitle(String),
    UpdateLocation(String),
    UpdateLink(String),
    UpdateCompany(String),
    UpdateDate(String),
    Submit,
    Close,
    Open,
}

impl Component for AddApplicationModal {
    type Message = Msg;
    type Properties = ApplicationModalProps;

    fn create(ctx: &Context<Self>) -> Self {
        log::info!("app_id: {}", ctx.props().application_id);
        Self { application: Application {application_id: ctx.props().application_id.to_string(), company: "".to_string(), status: Status::from_str("Pending"), job_title: "".to_string(), location: "".to_string(), application_date: "".to_string(), link:  "".to_string(), tasks: Vec::new()}, is_open: ctx.props().is_open.clone() }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.is_open = ctx.props().is_open;
        self.application.application_id = ctx.props().application_id.to_string();
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // Msg::UpdateApplicationId(id) => {
            //     self.application.application_id = id;
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
            Msg::Submit => {
                ctx.props().on_submit.emit(self.application.clone());
                ctx.props().on_close.emit(());
                // Clear input fields
                self.application.application_id.clear();
                self.application.application_date.clear();
                self.application.company.clear();
                self.application.status = Status::from_str("Pending");
                self.application.job_title.clear();
                self.application.location.clear();
                self.application.link.clear();
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
            Msg::UpdateLink(link) => {
                self.application.link = link;
                true
            }
            Msg::UpdateCompany(company) => {
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
            return html! {};
        }

        let link = ctx.link();
        html! {
            <div class="modal">
                <div class="modal-content">
                    <h2>{ "Add Application" }</h2>
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
                            placeholder="Job Title"
                            value={self.application.job_title.clone()}
                            oninput={link.callback(|e: InputEvent| {
                                let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                Msg::UpdateTitle(value)
                            })}
                        />
                    </div>
                    <div class="input-container">
                        <input
                            placeholder="Location"
                            value={self.application.location.clone()}
                            oninput={link.callback(|e: InputEvent| {
                                let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                Msg::UpdateLocation(value)
                            })}
                        />
                    </div>
                    <div class="input-container">
                        <input
                            placeholder="Date of Application"
                            value={self.application.application_date.clone()}
                            oninput={link.callback(|e: InputEvent| {
                                let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                Msg::UpdateDate(value)
                            })}
                        />
                    </div>
                    <div class="input-container">
                        <input
                            placeholder="Application Link"
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