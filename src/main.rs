use rocket::{launch, get, serde::json::Json, routes};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    message: String,
}

#[get("/")]
fn index() -> Json<Response> {
    Json(Response {
        message: "Hello World".to_string(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            index,
        ])
}
