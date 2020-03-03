use http::header::HeaderValue;
use warp::Filter;
use warp::http::StatusCode;
use warp::reply::Reply;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::convert::Infallible;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
	name: String,
}

fn json_body() -> impl Filter<Extract = (User,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub async fn join_game(user: User) -> Result<impl warp::Reply, Infallible> {
	// create a new Uuid, and send it back to the user

    let mut response = warp::reply::json(game::Game::new().join(&user.name)).into_response();
    response.headers_mut()
        .insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));

    Ok(response)
}

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_methods(vec!["POST"])
        .allow_headers(vec!["Content-Type"])
        .allow_any_origin();
    // main page
    let root = warp::path::end()
        .map(|| format!("Welcome to the HiCard"));

    // poll for turn, get
    let game_get = warp::path!("game")
    	.and(warp::get())
    	.map(|| format!("got game"));

    // join game with post
    let game_post = warp::path!("game")
    	.and(warp::post())
    	.and(json_body())
    	.and_then(join_game)
        .with(cors);

    // join game with post
    let move_post = warp::path!("move")
    	.and(warp::post())
    	.map(warp::reply);

    // wait for winner result
    let result = warp::path!("results")
    	.map(|| format!("show results"));

    let routes = root
    	.or(game_get)
    	.or(game_post)
    	.or(move_post)
    	.or(result);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
    println!("Serving at http://127.0.0.1:3030")
}
