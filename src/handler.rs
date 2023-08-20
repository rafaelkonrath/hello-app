use crate::{model::UserNameModel, model::UsersModel, schema::CreateUserSchema, AppState};
use actix_web::{get, put, web, HttpResponse, Responder};
use chrono::{Datelike, NaiveDate, Utc};
use serde_json::json;


pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("")
        .service(health_checker_handler)
        .service(create_user_handler)
        .service(get_user_handler);

    conf.service(scope);
}

#[get("/health")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Health";

    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}

#[put("/hello/{username}")]
async fn create_user_handler(
    path: web::Path<String>,
    body: web::Json<CreateUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let username = path.into_inner();

    if !&username.chars().all(|x| x.is_ascii_alphabetic()) {
        println!("ascii_alphabetic [{}]", username);
        return HttpResponse::NoContent().json("");
    }

    if NaiveDate::parse_from_str(&body.date_of_birth.as_str(), "%Y-%m-%d").is_err() {
        return HttpResponse::NoContent().json("");
    }

    let today = Utc::now();
    let datetime = NaiveDate::parse_from_str(&body.date_of_birth.as_str(), "%Y-%m-%d")
        .unwrap()
        .and_hms_opt(19, 32, 33)
        .unwrap()
        .and_utc();

    let diff = today.signed_duration_since(datetime);
    let days = diff.num_days();
    if days == 0 || days < 1 {
        return HttpResponse::NoContent().json("");
    }

    let query_result =
        sqlx::query_as::<_, UserNameModel>("SELECT username FROM users WHERE username = $1")
            .bind(&username)
            .fetch_one(&data.db)
            .await;

    if query_result.is_err() {
        let query_insert = sqlx::query_as::<_, UsersModel>(
            r#"INSERT INTO users(username, date_of_birth) VALUES($1, $2)"#,
        )
        .bind(&username)
        .bind(&body.date_of_birth)
        .fetch_all(&data.db)
        .await;

        if query_insert.is_err() {
            HttpResponse::NoContent().json("");
        }
    } else {
        let query_update = sqlx::query_as::<_, UsersModel>(
            r#"UPDATE users SET username = $1, date_of_birth = $2 WHERE username = $1"#,
        )
        .bind(&username)
        .bind(&body.date_of_birth)
        .fetch_all(&data.db)
        .await;

        if query_update.is_err() {
            HttpResponse::NoContent().json("");
        }
    }

    HttpResponse::NoContent().json("")
}

#[get("/hello/{username}")]
pub async fn get_user_handler(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let username = path.into_inner();

    let query_result = sqlx::query_as::<_, UsersModel>(
        r#"SELECT username, date_of_birth FROM users WHERE username = $1"#,
    )
    .bind(&username)
    .fetch_one(&data.db)
    .await;

    match &query_result {
        Ok(_user) => {
            let notes = query_result.unwrap();

            let date = Utc::now();
            let today = NaiveDate::from_ymd_opt(date.year(), date.month(), date.day()).unwrap();

            let date_of_birth =
                NaiveDate::parse_from_str(notes.date_of_birth.as_str(), "%Y-%m-%d").unwrap();

            let my_birthday =
                NaiveDate::from_ymd_opt(today.year(), date_of_birth.month(), date_of_birth.day())
                    .unwrap();

            if my_birthday == today {
                let message = format!("Hello, {}! Happy birthday!", username);
                let json_response = serde_json::json!({
                    "status": "success",
                    "message": message
                });

                return HttpResponse::Ok().json(json_response);
            } else {
                if my_birthday < today {
                    let my_birthday = NaiveDate::from_ymd_opt(
                        today.year() + 1,
                        date_of_birth.month(),
                        date_of_birth.day(),
                    )
                    .unwrap();

                    let diff = my_birthday.signed_duration_since(today);
                    let days = diff.num_days();

                    let message =
                        format!("Hello, {}! Your birthday is in {} day(s)", username, days);
                    let json_response = serde_json::json!({
                        "status": "success",
                        "message": message
                    });

                    return HttpResponse::Ok().json(json_response);
                } else {
                    let diff = my_birthday.signed_duration_since(today);
                    let days = diff.num_days();

                    let message =
                        format!("Hello, {}! Your birthday is in {} day(s)", username, days);
                    let json_response = serde_json::json!({
                        "status": "success",
                        "message": message
                    });

                    return HttpResponse::Ok().json(json_response);
                }
            }
        }
        Err(_) => {
            let message = format!("Username: {} not found", username);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail", "message": message}));
        }
    }

}
