use yew::{html, Callback, ClickEvent, Component, ComponentLink, Html, ShouldRender};

//mod play;
mod register;

pub(crate) use register::Register;
//pub(crate) use play::Play;


pub(crate) struct App {
    clicked: bool,
    onclick: Callback<ClickEvent>,
}

pub(crate) enum Msg {
    Click,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            clicked: false,
            onclick: link.callback(|_| Msg::Click),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.clicked = true;
                true // Indicate that the Component should re-render
            }
        }
    }

    fn view(&self) -> Html {
        let button_text = if self.clicked { "Clicked!" } else { "Click me!" };

        html! {
            <body>
            <h1>{"High Card Draw"}</h1>
            <div>
                <Register />
            </div>
            <div>
                <button onclick=&self.onclick>{ button_text }</button>
            </div>
            </body>
        }
    }
}

