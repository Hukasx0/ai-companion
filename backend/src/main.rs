use actix_web::{get, App, HttpResponse, HttpServer};
use reqwest::Client;

static OPENAI_API_KEY: &str = "xyz";

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

#[get("/api/testPrompt")]
async fn test_prompt() -> HttpResponse {
    let client = Client::new();
    let prompt = client.get("https://api.openai.com/v1/completions")
                    .header("Authorization", format!("Bearer {}", OPENAI_API_KEY))
                    .header("Content-Type", "application/json")
                    .body("ping");
    match prompt.send().await {
        Ok(response) => {
            HttpResponse::Ok().body(format!("OpenAI Api response: {}", response.text().await.unwrap()))
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("{}", e))
        }
    }
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
            .service(test_prompt)
    })
    .bind((hostname, port))?
    .run()
    .await
}
