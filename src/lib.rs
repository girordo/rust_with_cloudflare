use serde::{Deserialize, Serialize};
use worker::*;

use futures::future::join_all;
use serde_json::to_string;

struct SharedData {
    name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct AnimalRescue {
    id: u8,
    name: String,
    age: u8,
    species: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct AnimalRescueUpdate {
    name: String,
    age: u8,
    species: String,
}

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    let shared_data = SharedData {
        name: "Rusty".to_string(),
    };

    let router = Router::with_data(shared_data);

    router.run(req, env).await
}
