use yew::prelude::*;

use game::Player;

pub enum Msg {
    X,
}

pub(crate) struct Register {
    properties: (),
    link: ComponentLink<Self>,
    player: Option<Player>
}

impl Component for Register {
    type Properties = ();
    type Message = Msg;

    fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Register {
        Register { properties, link, player: None }
    }

    fn update(&mut self, message: Msg) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match &self.player {
            Some(player) => {
                html!{
                    <p>{format!("Welcome {}", player.name)}</p>
                }
            }
            None => {
                html!{
                    <>
                        <p>{"Join the game"}</p>
                        <input id="join" />
                    </>
                }
            }
        }
    }
}
