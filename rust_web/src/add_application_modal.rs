use yew::prelude::*;

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
                self.is_open = false;
                true
            }
            ApplicationModalMsg::Close => {
                ctx.props().on_close.emit(());
                self.is_open = false;
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