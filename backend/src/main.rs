mod ai;
mod config;
mod db_helpers;
mod handlers;
mod jwt_auth;
mod response;
mod token;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, middleware, web, App, HttpServer};
use config::Config;
use dotenv::dotenv;
use llm_chain::executor;
use llm_chain::options::ModelRef;
use llm_chain_openai::chatgpt::{Executor, Model};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
    env: Config,
    redis_client: redis::Client,
    chatgpt: Executor,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=debug");
    }
    dotenv().ok();
    env_logger::init();

    let config = Config::init();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {}", err);
            std::process::exit(1);
        }
    };

    let redis_client = match redis::Client::open(config.redis_url.to_owned()) {
        Ok(client) => {
            println!("✅ Connection to the redis is successful!");
            client
        }
        Err(e) => {
            println!("🔥 Error connecting to Redis: {}", e);
            std::process::exit(1);
        }
    };

    let mut opts_builder = llm_chain::options::Options::builder();
    opts_builder.add_option(llm_chain::options::Opt::Model(ModelRef::from(
        Model::Gpt35Turbo,
        //Model::Gpt4,   // Works but is slower, obviously.
    )));

    let exec = executor!(chatgpt, opts_builder.build()).unwrap();

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&config.client_origin)
            .allowed_methods(vec!["GET", "POST", "PATCH"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.to_owned(),
                env: config.to_owned(),
                redis_client: redis_client.to_owned(),
                chatgpt: exec.to_owned(),
            }))
            .wrap(middleware::Compress::default())
            .service(
                web::scope("/api")
                    .configure(handlers::auth::config)
                    .configure(handlers::user::config)
                    .configure(handlers::collections::config)
                    .configure(handlers::ai::config),
            )
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
