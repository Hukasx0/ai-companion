use actix_web::{get, App, HttpResponse, HttpServer};

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body(include_str!("../../dist/index.html"))
}

#[get("/vite.svg")]
async fn vite_logo() -> HttpResponse {
    HttpResponse::Ok().content_type("image/svg+xml").body(include_str!("../../dist/vite.svg"))
}

#[get("/assets/index-4rust.js")]
async fn js() -> HttpResponse {
    HttpResponse::Ok().content_type("application/javascript").body(include_str!("../../dist/assets/index-4rust.js"))
}

#[get("/assets/index-4rust.css")]
async fn css() -> HttpResponse {
    HttpResponse::Ok().content_type("text/css").body(include_str!("../../dist/assets/index-4rust.css"))
}

#[get("/assets/react-4rust.svg")]
async fn react_logo() -> HttpResponse {
    HttpResponse::Ok().content_type("image/svg+xml").body(include_str!("../../dist/assets/react-4rust.svg"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let port: u16 = 3000;
    let hostname: &str = "127.0.0.1";

    println!("Started Rust backend server at:\n -> http://{}:{}/", hostname, port);
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(js)
            .service(css)
            .service(vite_logo)
            .service(react_logo)
    })
    .bind((hostname, port))?
    .run()
    .await
}
