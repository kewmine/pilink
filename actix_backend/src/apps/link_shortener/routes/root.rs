use actix_web::{get, Responder, HttpResponse};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../webpages/index.html"))
}

#[get("/404")]
async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("you are lost x_x\n")
}
