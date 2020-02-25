
use warp::Filter;

#[tokio::main]
async fn main() {
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
    	.map(|| format!("join game"));

    // join game with post
    let move_post = warp::path!("move")
    	.and(warp::post())
    	.map(|| format!("make a move"));

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
}

