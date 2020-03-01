use yew::prelude::*;

use game::{Game, Player};

pub enum Msg {
    Register,
    RegisterComplete(Player),
    Input(String),
    Failed,
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
                    let fetcher = yew::services::FetchService::new();
                    let register_req = http::Request::post("http://localhost:8080/game")
                        .body(&self.input)
                        .expect("failed to build request");
                    let register_callback = self.link.callback(|response: http::Response<yew::format::Json<Result<serde_json::Value, anyhow::Error>>>| {
                        let player: Result<Player, anyhow::Error> =(|response: http::Response<yew::format::Json<Result<serde_json::Value, anyhow::Error>>>| {
                            let (meta, yew::format::Json(result)) = response.into_parts();
                            if meta.status.is_success() {
                                let value = result?;
                                let player = (|value: serde_json::Value| {
                                    // Workaround until we implement deserialize for player
                                    let name = value["name"].as_str()?.to_string();
                                    let id = uuid:: Uuid::parse_str(&value["id"].as_str()?.to_string()).ok()?;

                                    Some (Player { name, id, card: None })
                                })(value).ok_or_else(||anyhow::anyhow!("failed to create player"))?;
                                Ok(player)

                            } else {
                                Err(result.unwrap_err())
                            }
                        })(response);
                        match player {
                            Ok(player) => {
                                console.log(&format!("Got a player: {:?}", player));
                                Msg::RegisterComplete(player)
                            }
                            Err(err) => {
                                console.error(&format!("{}", err));
                                Msg::Register
                            }
                        }
                    });
                    let task = fetcher.fetch(
                        register_req,
                        register_callback,
                    );
                    false
                }
            }
            Msg::RegisterComplete(player) => {
                self.player.replace(player);
                true
            }
            Msg::Failed => false,
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
