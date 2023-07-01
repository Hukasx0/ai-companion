use actix_web::{get, App, HttpResponse, HttpServer};
use rs_llama_cpp::{gpt_params_c, run_inference, str_to_mut_i8};


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
    let params: gpt_params_c = {
        gpt_params_c {
            model: str_to_mut_i8("models/llama.7b.wizard-1.0.ggml_v3.q5_1.bin"),
            prompt: str_to_mut_i8("Hello, who are you?"),
            ..Default::default()
        }
    };

    let response = String::new();

    run_inference(params, |token| {
        println!("{}", token);
        if token.ends_with("\n") {
            return false;
        }

        return true;
    });
    return HttpResponse::Ok().body(response);
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
