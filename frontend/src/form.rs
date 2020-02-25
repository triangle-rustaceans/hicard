use yew::prelude::*;

pub struct RegisterForm {
    value: String,
    link: ComponentLink<Self>,
}

pub enum RegisterMsg {
    RegisterAs(String)
}

impl Component for RegisterForm {
    type Properties = ();
    type Msg = RegisterMsg;

    fn create(props: (), link: ComponentLink<Self>) -> Self {
        RegisterForm {
            value: String::new(),
            link,
        }
    }
}

impl Renderable<RegisterForm> for RegisterForm {
    fn view(&self) -> Html<Self> {
        html!{
            <>
                <label for="name">Name</label>
                <input id="name"/>
            </>
        };
    }
}
