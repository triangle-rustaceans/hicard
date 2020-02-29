use yew::prelude::*;

use game::{Game, Player};

pub enum Msg {
    Register,
    Input(String),
}

#[derive(Clone, Properties)]
pub struct Props {
}

pub(crate) struct Register {
    properties: Props,
    link: ComponentLink<Self>,
    player: Option<Player>,
    input: String,
}

impl Component for Register {
    type Properties = Props;
    type Message = Msg;

    fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Register {
        Register { properties, link, player: None, input: String::new() }
    }

    fn update(&mut self, message: Msg) -> ShouldRender {
        match message {
            Msg::Register => {
                let mut console = yew::services::ConsoleService::new();
                console.log(&format!("Registering with name {:?}", self.input));
                if self.input.is_empty() {
                    console.log("EMPTY");

                    false
                } else {
                    /*
                    let fetcher = yew::services::FetchService::new();
                    let register_req = http::Request::post("http://localhost:8080/game")
                        .body(&self.input)
                        .expect("failed to build request");
                    let task = fetcher.fetch(
                        register_req,
                        self.link.callback(|response: http::Response<Result<String, anyhow::Error>>| {
                                if response.status().is_success() {
                                let body = response.into_body();
                                let player: Player = from_json(body)?
                                    b.
                                self.player_callback(player)
                            }
                        })
                    )
                    */
                    console.log("NOT EMPTY");
                    let mut game = Game::new();
                    console.log("GOT game");
                    let playerref = game.join(&self.input);
                    console.log("GOT playerref");
                    self.player = Some(playerref.clone());
                    console.log(&format!("set self.player to {:?}", playerref));
                    true
                }
            }
            Msg::Input(input) => {
                self.input = input;
                false
            }
        }
    }

    fn view(&self) -> Html {
        let mut console = yew::services::ConsoleService::new();
        console.log("RENDERING");

        match &self.player {
            Some(player) => {
                html!{
                    <p>{format!("Welcome {}", player.name)}</p>
                }
            }
            None => {
                let onclick = self.link.callback(|click| {
                    let mut console = yew::services::ConsoleService::new();
                    console.log("Got a click");
                    Msg::Register
                });
                let oninput = self.link.callback(|input: InputData| {
                    let mut console = yew::services::ConsoleService::new();
                    console.log(&format!("Got input: {:?}", input));
                    Msg::Input(input.value)
                });
                html!{
                    <>
                        <p>{"Join the game"}</p>
                        <input type="text" oninput=oninput id="name"/>
                        <button title="Join" onclick=onclick>{"Join"}</button>
                    </>
                }
            }
        }
    }
}
