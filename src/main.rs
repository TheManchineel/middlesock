use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use reqwest;
use rocket::get;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use std::time::{SystemTime, UNIX_EPOCH};

#[macro_use]
extern crate rocket;

#[derive(Deserialize)]
struct Config {
    authentik_base_url: String,
    authentik_api_key: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Pagination {
    count: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Users {
    pagination: Pagination,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Cords {
    x_cord: f64,
    y_cord: u128,
}

#[derive(Debug, PartialEq, FromFormField)]
enum Action {
    #[allow(non_camel_case_types)] // this matches the query parameter
    login,
    #[allow(non_camel_case_types)]
    login_failed,
}

async fn get_users(base_url: String, api_key: String) -> u64 {
    let response = reqwest::Client::new()
        .get(&format!("{}api/v3/core/users/", base_url))
        .header("Authorization", "Bearer ".to_owned() + api_key.as_str())
        .send()
        .await
        .unwrap();

    // I only need the count of users, so I don't need to deserialize the whole JSON
    let json: Users = response.json::<Users>().await.unwrap();
    json.pagination.count
}

#[get("/api/v3/core/users")]
async fn user_count(config: &State<Config>) -> Json<Users> {
    let user_count = get_users(
        config.authentik_base_url.clone(),
        config.authentik_api_key.clone(),
    )
    .await;
    let pagination = Pagination { count: user_count };
    let users = Users { pagination };
    Json(users)
}

async fn get_events(base_url: String, api_key: String, action: Action) -> Vec<Cords> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let yesterday_timestamp = timestamp - 86400000; // 86400000 = 1 day in milliseconds

    let url = format!(
        "{}api/v3/events/events/per_month/?action={:?}&query={{}}",
        base_url, action
    );
    let response = reqwest::Client::new()
        .get(&url)
        .header("Authorization", "Bearer ".to_owned() + api_key.as_str())
        .send()
        .await
        .unwrap();

    let json: Vec<Cords> = response.json::<Vec<Cords>>().await.unwrap();

    let mut y_cord = 0;

    // add all y_cords for each JSON object where x_cord is after yesterday
    for i in 0..json.len() {
        if json[i].x_cord >= yesterday_timestamp as f64 {
            println!(
                "Adding y_cord: {} for x_cord: {}",
                json[i].y_cord, json[i].x_cord
            );

            y_cord += json[i].y_cord;
        }
    }

    vec![Cords {
        x_cord: timestamp as f64,
        y_cord: y_cord,
    }]
}

#[get("/api/v3/events/events/per_month?<action>")]
async fn events_per_month(config: &State<Config>, action: Action) -> Json<Vec<Cords>> {
    let cords: Vec<Cords> = get_events(
        config.authentik_base_url.clone(),
        config.authentik_api_key.clone(),
        action,
    )
    .await;
    Json(cords)
}

#[launch]
fn rocket() -> _ {
    let config = Figment::new()
        .merge(Toml::file("config.toml"))
        .merge(Env::raw().only(&["AUTHENTIK_BASE_URL", "AUTHENTIK_API_KEY"]))
        .extract::<Config>()
        .unwrap();

    rocket::build()
        .mount("/", routes![user_count, events_per_month])
        .manage(Config::from(config))
}
