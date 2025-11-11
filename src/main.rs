mod db;
mod handlers;
mod middleware;
mod models;
mod utils;

use actix_cors::Cors;
use actix_files as fs;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    let pool = db::create_pool()
        .await
        .expect("Failed to create database pool");

    log::info!("🚀 Starting Alpha Bank Server at http://{}:{}", host, port);
    log::info!("📊 Database connected successfully");
    log::info!("🌐 Frontend available at http://{}:{}", host, port);
    log::info!("🔌 API available at http://{}:{}/api", host, port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            // API Routes
            // Public routes
            .service(
                web::scope("/api/auth")
                    .route("/register", web::post().to(handlers::auth::register))
                    .route("/login", web::post().to(handlers::auth::login))
                    .route("/forgot-password", web::post().to(handlers::auth::forgot_password)),
            )
            // Protected routes
            .service(
                web::scope("/api")
                    .wrap(middleware::auth::Auth)
                    // User profile
                    .route("/me", web::get().to(handlers::auth::me))
                    .route("/me", web::put().to(handlers::auth::update_profile))
                    .route("/auth/change-password", web::post().to(handlers::auth::change_password))
                    // Transactions
                    .service(
                        web::scope("/transactions")
                            .route("", web::get().to(handlers::transactions::get_all))
                            .route("", web::post().to(handlers::transactions::create))
                            .route("/{id}", web::get().to(handlers::transactions::get_by_id))
                            .route("/{id}", web::put().to(handlers::transactions::update))
                            .route("/{id}", web::delete().to(handlers::transactions::delete)),
                    )
                    // Categories
                    .service(
                        web::scope("/categories")
                            .route("", web::get().to(handlers::categories::get_all))
                            .route("", web::post().to(handlers::categories::create))
                            .route("/{id}", web::put().to(handlers::categories::update))
                            .route("/{id}", web::delete().to(handlers::categories::delete)),
                    )
                    // Goals
                    .service(
                        web::scope("/goals")
                            .route("", web::get().to(handlers::goals::get_all))
                            .route("", web::post().to(handlers::goals::create))
                            .route("/{id}", web::get().to(handlers::goals::get_by_id))
                            .route("/{id}", web::put().to(handlers::goals::update))
                            .route("/{id}", web::delete().to(handlers::goals::delete))
                            .route("/{id}/progress", web::post().to(handlers::goals::add_progress)),
                    )
                    // Recurring Transactions
                    .service(
                        web::scope("/recurring")
                            .route("", web::get().to(handlers::recurring::get_all))
                            .route("", web::post().to(handlers::recurring::create))
                            .route("/{id}", web::put().to(handlers::recurring::update))
                            .route("/{id}", web::delete().to(handlers::recurring::delete))
                            .route("/generate", web::post().to(handlers::recurring::generate_pending)),
                    )
                    // Notifications
                    .service(
                        web::scope("/notifications")
                            .route("", web::get().to(handlers::notifications::get_all))
                            .route("", web::post().to(handlers::notifications::create))
                            .route("/{id}/read", web::put().to(handlers::notifications::mark_as_read))
                            .route("/{id}", web::delete().to(handlers::notifications::delete)),
                    ),
            )
            // Health check
            .route("/health", web::get().to(|| async { HttpResponse::Ok().body("OK") }))
            // Serve static files (Frontend)
            .service(fs::Files::new("/src", "./static/src").show_files_listing())
            .service(
                fs::Files::new("/", "./static")
                    .index_file("index.html")
                    .show_files_listing()
            )
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}

