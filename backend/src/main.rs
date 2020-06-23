use http::header::HeaderValue;
use warp::Filter;
use warp::reply::Reply;
use std::convert::Infallible;
use uuid::Uuid;

use std::net::{SocketAddr, Ipv6Addr};
use game::{Game, Player};


fn json_body() -> impl Filter<Extract = (Player,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub async fn join_game(player: Player, aGame: Game) -> Result<impl warp::Reply, Infallible> {
	// create a new Uuid, and send it back to the player

    let mut response = warp::reply::json(aGame.join(&player.name)).into_response();
    response.headers_mut()
        .insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));

    Ok(response)
}

pub async fn play_game(player_id: &Uuid, aGame: Game) -> Result<impl warp::Reply, Infallible> {
    // given a Uuid, make sure player is current player and get a card

    let mut response = warp::reply::json(aGame::play().join(player_id)).into_response();
    response.headers_mut()
        .insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));

    Ok(response)
}

#[tokio::main]
async fn main() {
    let game = Game::new();
    let game_filter = warp::any().map(move || game.clone());
    let socket: SocketAddr = (Ipv6Addr::UNSPECIFIED, 3030).into();
    let cors = warp::cors()
        .allow_methods(vec!["POST"])
        .allow_headers(vec!["Content-Type"])
        .allow_any_origin();
    // main page
    let root = warp::path::end()
        .map(|| format!("Welcome to the HiCard"));

    // poll for turn, get current status for all players, cards for players, and who's turn is next
    let game_get = warp::path!("game")
    	.and(warp::get())
    	.map(|| format!("got game"));

    // join game with post, get Player
    let game_post = warp::path!("game")
    	.and(warp::post())
    	.and(json_body())
        .and(game_filter.clone())
    	.and_then(join_game)
        .with(cors);

    // join game with post
    let move_post = warp::path!("move")
    	.and(warp::post())
        .and(json_body())
        .and(game_filter.clone())
        .and_then(play_game)
        .with(cors);

    // wait for winner result
    let result = warp::path!("results")
    	.map(|| format!("show results"));

    let routes = root
    	.or(game_get)
    	.or(game_post)
    	.or(move_post)
    	.or(result);

    println!("Serving at http://[::0]:3030");
    warp::serve(routes)
        .run(socket)
        .await;
}
