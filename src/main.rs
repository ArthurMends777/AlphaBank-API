use actix_web::{get, App, HttpServer, Responder};

// #[get("/")] é um macro que registra esta função para responder a requisições GET na rota "/"
#[get("/")]
async fn hello() -> impl Responder {
    "AphaBank!" 
}

// #[actix_web::main] é um macro que configura o runtime assíncrono (async) do Actix.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Cria e configura o servidor HTTP
    HttpServer::new(|| {
        // App::new() cria a aplicação, onde você registra as rotas e middlewares
        App::new()
            // .service() registra o nosso 'handler' de rota 'hello'
            .service(hello)
    })
    // Define o endereço e a porta para escutar (bind)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}