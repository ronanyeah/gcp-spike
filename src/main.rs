use actix_web::{web, App, HttpResponse, HttpServer};

async fn ping() -> actix_web::Result<HttpResponse> {
    let millis = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .map_err(|_| actix_web::HttpResponse::InternalServerError())?
        .as_millis()
        .to_string();

    Ok(HttpResponse::with_body(
        actix_web::http::StatusCode::OK,
        actix_web::body::Body::from_message(millis),
    ))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().route("/", web::get().to(ping)))
        .bind(("0.0.0.0", 8888 as u16))?
        .run()
        .await
}
