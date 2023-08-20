mod handler;
mod model;
mod schema;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use actix_web_prom::PrometheusMetricsBuilder;

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    println!("🚀 Server started successfully");
    
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();

        HttpServer::new(move || {
            let cors = Cors::default()
                .allowed_methods(vec!["GET", "PUT"])
                .allowed_headers(vec![
                    header::CONTENT_TYPE,
                    header::ACCEPT,
                ])
                .supports_credentials();
            App::new()
                .app_data(web::Data::new(AppState { db: pool.clone() }))
                .configure(handler::config)
                .wrap(prometheus.clone())
                .wrap(cors)
                .wrap(Logger::default())
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await?;
    
    Ok(())

}


#[cfg(test)]
mod tests {
    use actix_web::{body::to_bytes, dev::Service, http, test, web, App, Error};
    use crate::{schema::CreateUserSchema, AppState};
    use sqlx::postgres::PgPoolOptions;
    use dotenv::dotenv;

    use super::*;

    #[actix_web::test]
    async fn health() -> Result<(), Error> {
        let app = App::new()
            .configure(handler::config);
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = app.call(req).await?;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body_bytes = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(body_bytes, r#"{"message":"Health","status":"success"}"#);

        Ok(())
    }
    
    #[actix_web::test]
    async fn create_user_handler() -> Result<(), Error> {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = match PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await
            {
                Ok(pool) => {
                    println!("✅Connection to the database is successful!");
                    pool
                }
                Err(err) => {
                    println!("🔥 Failed to connect to the database: {:?}", err);
                    std::process::exit(1);
                }
            };

        let app = App::new()
            .configure(handler::config)
            .app_data(web::Data::new(AppState { db: pool.clone() }));
        let app = test::init_service(app).await;

        let req = test::TestRequest::put().uri("/hello/test")
            .set_json(CreateUserSchema {
                date_of_birth: "1970-01-01".to_string()
            })
            .to_request();
        let resp = app.call(req).await?;

        assert_eq!(resp.status(), http::StatusCode::NO_CONTENT);

        Ok(())
    }

    #[actix_web::test]
    async fn get_user_handler() -> Result<(), Error> {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = match PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await
            {
                Ok(pool) => {
                    println!("✅Connection to the database is successful!");
                    pool
                }
                Err(err) => {
                    println!("🔥 Failed to connect to the database: {:?}", err);
                    std::process::exit(1);
                }
            };

        let app = App::new()
            .configure(handler::config)
            .app_data(web::Data::new(AppState { db: pool.clone() }));
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/hello/test")
            .to_request();
        let resp = app.call(req).await?;

        assert_eq!(resp.status(), http::StatusCode::OK);

        Ok(())
    }

}

