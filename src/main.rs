use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Utc};
use std::time::SystemTime;

#[get("/")]
async fn daytime() -> impl Responder {
    let system_time = SystemTime::now();
    let datetime :DateTime<Utc> = system_time.into();
    HttpResponse::Ok().body(datetime.format("%a %b %d %T %Z %Y").to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(daytime)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}