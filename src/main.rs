use yew::{
    html, 
    Callback, 
    ClickEvent, 
    Component, 
    ComponentLink, 
    Html, 
    ShouldRender,
    InputData,
};

struct App {
    text: String,
    todo: Vec<String>,
    on_add: Callback<ClickEvent>,
    on_change: Callback<InputData>,
    link: ComponentLink<Self>
}

enum Msg {
    Add,
    Change(String),
    Remove(usize)
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            text: "".to_string(),
            todo: vec![],
            on_add: link.callback(|_| Msg::Add),
            on_change: link.callback(|e: InputData| Msg::Change(e.value)),
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                let text = self.text.clone();
                self.text = "".to_string();
                self.todo.push(text);
                true // Indicate that the Component should re-render
            },
            Msg::Change(val) => {
                self.text = val;
                true // Indicate that the Component should re-render
            },
            Msg::Remove(i) => {
                self.todo.remove(i);
                true
            }

        }
    }

    fn view(&self) -> Html {
        
        let list_item = |(i, item): (usize, &String)| {
            html!{
                <li class="item">
                {format!("{} ", &item)}
                <button onclick=self.link.callback(move |_| Msg::Remove(i))>{"x"}</button>
                </li>
            }
        };

        html! {
            <>
            <input oninput=&self.on_change value=&self.text />
            <button onclick=&self.on_add>{ "Add" }</button>
            <ul class="list">
                {for self.todo.iter().enumerate().map(list_item)}
            </ul>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
