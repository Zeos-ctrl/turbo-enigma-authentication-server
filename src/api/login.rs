use super::super::auth::account::{Login,Verify};
use super::super::auth::jwt::JwtToken;
use super::super::auth::otp::verify_totp;
use super::super::connections::connect::Pool;

use rocket::State;
use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::json::{json, Value};
use log::{error, info};

use super::response::ApiResponse;

#[post("/login", data = "<login>")]
pub async fn login(pool: &State<Pool>, login: Form<Login>) -> ApiResponse{
    //Secret for otp
    let secret = String::from("KRSXG5CTMVRXEZLUKN2XAZLSKNSWG4TFOQ");

    //Checks if the otp is correct and if it is logs in the user
    if verify_totp(secret,&login.otp){
        let user = sqlx::query!(
            r#"
            SELECT *
            FROM users
            WHERE username = ?;
            "#,
            &login.username
            )
            .fetch_one(&pool.0)
            .await;

        match user {
            Ok(user) => {
                if login.verify_password(&user.password){
                    let token = JwtToken::encode(&user.uuid.to_string());
                    info!("Logged in user: {}", &login.username);
                    ApiResponse {
                        auth_token: token.to_string(),
                        status: Status::Ok
                    }
                }else{
                    error!("Incorrect login credentials for: {}", &login.username);
                    ApiResponse {
                        auth_token: format!("none"),
                        status: Status::Unauthorized
                    }
                }
            },
            Err(_) => ApiResponse {
                            auth_token: format!("none"),
                            status: Status::InternalServerError
                        }
        }
    }else{
        error!("Incorrect OTP during login for: {}", &login.username);
        ApiResponse {
            auth_token: format!("none"),
            status: Status::BadRequest
        }
    }
}
