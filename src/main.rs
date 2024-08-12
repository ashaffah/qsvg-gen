pub mod test;

#[allow(unused_imports)]
#[allow(dead_code)]

#[macro_use]
extern crate rocket;

use qirust::helper::generate_svg_string;
use std::{ collections::HashMap, path::{ Path, PathBuf } };
use rocket::{
    fairing::{ Fairing, Info, Kind },
    Request,
    Response,
    fs::NamedFile,
    http::Header,
    response::Redirect,
    serde::{ json::Json, Deserialize, Serialize },
};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses for all routes.",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(
            Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS")
        );
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

// Define a struct to represent the incoming JSON
#[derive(Serialize, Deserialize)]
struct SvgData {
    content: String,
}

#[get("/")]
fn index() -> Redirect {
    let redirect = Redirect::to(uri!("/home"));
    redirect
}

#[get("/home")]
async fn home() -> Option<NamedFile> {
    NamedFile::open("src/template/html/index.html").await.ok()
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("src/template/").join(file)).await.ok()
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
    rocket
        ::build()
        .mount("/", routes![index, home, files])
        .mount("/api", routes![get_svg, get_svg_post])
        .attach(CORS)
}
