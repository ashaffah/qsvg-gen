#[allow(unused_imports)]
#[allow(dead_code)]

#[macro_use] extern crate rocket;

use qirust::helper::generate_svg_string;
use rocket::data::ByteUnit;
use rocket::Data;
use rocket::serde::json::Json;
use std::collections::HashMap;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};

#[get("/svg?<data>")]
async fn get_svg(data: String) -> Json<HashMap<String, String>> {
    let svg_string = generate_svg_string(&data);
    let mut map = HashMap::new();
    map.insert("svg".to_string(), svg_string);
    Json(map)
}

#[post("/svg", data = "<content>")]
async fn get_svg_post(content: Data<'_>) -> Json<HashMap<String, String>> {
    let param_string: String = content.open(ByteUnit::default()).into_string().await.unwrap().to_string();
    let svg_string = generate_svg_string(&param_string);
    let mut map = HashMap::new();
    map.insert("svg".to_string(), svg_string);
    Json(map)
}

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(vec![Method::Get, Method::Post].into_iter().map(From::from).collect())
        .allow_credentials(true)
        .to_cors().unwrap();

    rocket::build()
        .mount("/api", routes![get_svg, get_svg_post])
        .attach(cors)
}