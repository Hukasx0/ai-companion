use actix_web::{get, App, HttpResponse, HttpServer};
use llm::Model;
use std::io::Write;

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

    // https://github.com/rustformers/llm
    // https://docs.rs/llm/latest/llm/

    let llama = llm::load::<llm::models::Llama>(
        // https://huggingface.co/TehVenom/Pygmalion-7b-4bit-Q4_1-GGML/tree/main
        std::path::Path::new("models/Pygmalion-7b-4bit-Q4_1-GGML-V2.bin"),
        Default::default(),
        llm::load_progress_callback_stdout
    )
    .unwrap_or_else(|err| panic!("Failed to load model: {err}"));
    
    let mut session = llama.start_session(Default::default());
    let mut x = String::new();
    println!("Generating ai response...");
    let res = session.infer::<std::convert::Infallible>(
        &llama,
        &mut rand::thread_rng(),
        &llm::InferenceRequest {
            prompt: "Hello world in Python",
            ..Default::default()
        },
        &mut Default::default(),
        |t| {
            x = (x.clone()+t).to_string();
            print!("{t}");
            std::io::stdout().flush().unwrap();
            Ok(())
        }
    );
    
    match res {
        Ok(result) => println!("\n\nInference stats:\n{result}"),
        Err(err) => println!("\n{err}"),
    }
    return HttpResponse::Ok().body(x);
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
