#[allow(unused_imports)]
#[allow(dead_code)]

#[macro_use]
extern crate rocket;

use qirust::helper::generate_svg_string;
// use rocket::{ data::ToByteUnit, Data };
use rocket::serde::json::Json;
use std::collections::HashMap;
use rocket::http::Method;
use rocket_cors::{ AllowedOrigins, CorsOptions };
use rocket::serde::{ Deserialize, Serialize };
use rocket::response::content::RawHtml;

// Define a struct to represent the incoming JSON
#[derive(Serialize, Deserialize)]
struct SvgData {
    content: String,
}

#[get("/")]
fn index() -> RawHtml<&'static str> {
    let helo =
        "<div style=\"text-align:center\">
        <div style=\"font-size: 70px\">WELCOME</div>
        <div style=\"font-size: 40px\">ASHRAF SERVICE</div>
    </div>";
    RawHtml(helo)
}

#[get("/svg?<data>")]
async fn get_svg(data: String) -> Json<HashMap<String, String>> {
    let svg_string = generate_svg_string(&data);
    let mut map = HashMap::new();
    map.insert("svg".to_string(), svg_string);
    Json(map)
}

// result of qr code = "Hello World"
#[post("/svg", format = "json", data = "<svg_data>")]
async fn get_svg_post(svg_data: Json<SvgData>) -> Json<HashMap<String, String>> {
    let svg_string = generate_svg_string(&svg_data.content);
    let mut map = HashMap::new();
    map.insert("svg".to_string(), svg_string);
    Json(map)
}

// result of qr code = { "content": "Hello World" }
// #[post("/svg", data = "<data>")]
// async fn get_svg_post(data: Data<'_>) -> Json<HashMap<String, String>> {
//     let body = data.open((256).kibibytes()).into_bytes().await.unwrap();
//     let data_string = String::from_utf8_lossy(&body).to_string();
//     let svg_string = generate_svg_string(&data_string);
//     let mut map = HashMap::new();
//     map.insert("svg".to_string(), svg_string);
//     Json(map)
// }

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(vec![Method::Get, Method::Post].into_iter().map(From::from).collect())
        .allow_credentials(true)
        .to_cors()
        .unwrap();

    rocket
        ::build()
        .mount("/", routes![index])
        .mount("/api", routes![get_svg, get_svg_post])
        .attach(cors)
}
