use yew::prelude::*;
use crate::Application;

/// To-do list component that allows adding and removing tasks
pub struct TodoList {
    // tasks: Vec<String>,
    task: String,
    application: Application,
}

/// Properties for the to-do list component
#[derive(Properties, PartialEq)]
pub struct TodoProps {
    pub application: Application,
    pub tasks: Vec<String>,
    pub on_update: Callback<Application>,
}

/// Messages used to update the To-do list component
pub enum Msg {
    /// Used to add the task to the task list
    AddTask,
    /// Removes a task from the task list
    RemoveTask(usize),
    /// Updates the task on user input
    UpdateTask(String),
}

impl Component for TodoList {
    type Message = Msg;
    type Properties = TodoProps;

    /// Creates a new instance of the to-do list component
    fn create(ctx: &Context<Self>) -> Self {
        /*
        if ctx.props().tasks.len() > 0 {
            Self { tasks: ctx.props().tasks.clone(), task: "".to_string() }
        } else {
            Self { tasks: Vec::new(), task: "".to_string() }
        }
        */
        Self { task:"".to_string(), application: ctx.props().application.clone() }
    }

    /// Handles messages and updates components state
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddTask => {
                if !self.task.is_empty() {
                    self.application.tasks.push(self.task.clone());
                    self.task.clear();
                    ctx.props().on_update.emit(self.application.clone());
                    true
                } else {
                    false
                }
            }
            Msg::RemoveTask(index) => {
                if index < self.application.tasks.len() {
                    self.application.tasks.remove(index);
                    ctx.props().on_update.emit(self.application.clone());
                    true
                } else {
                    false
                }
            }
            Msg::UpdateTask(task) => {
                self.task = task;
                true
            }
        }
    }

    /// Renders the to-do list component
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="tasks-container">
                <h3>{ "Task List" }</h3>
                <ul class="task-list">
                    { for self.application.tasks.iter().enumerate().map(|(index, task)| html! {
                        <li key={index} class="task-item">
                            { task.clone() }
                            <button class="remove-task" onclick={link.callback(move |_| Msg::RemoveTask(index))}>{ "Remove" }</button>
                        </li>
                    }) }
                </ul>
                <div class="input-container">
                    <input
                        type="text"
                        value={self.task.clone()}
                        oninput={link.callback(|e: InputEvent| {
                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                            Msg::UpdateTask(input.value())
                        })}
                        placeholder="New task"
                    />
                    <button class="add-task" onclick={link.callback(|_| Msg::AddTask)}>{ "Add Task" }</button>
                </div>

            </div>
        }
    }
}