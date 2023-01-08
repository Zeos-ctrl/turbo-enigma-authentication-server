use super::super::auth::jwt::JwtToken;
use super::super::connections::connect::Pool;

use rocket::State;
use log::{error, info};

#[delete("/remove")]
pub async fn remove_account(pool: &State<Pool>, token: JwtToken) -> String{
    let decoded = JwtToken::decode(token.body).unwrap();
    let query = sqlx::query!(
        r#"
        DELETE FROM users
        WHERE uuid = ?;"#,
        &decoded.user_id
    )
    .execute(&pool.0)
    .await;

    match query {
        Ok(_query) => {
            info!("Removing account: {}", &decoded.user_id);
            format!("http://127.0.0.1/index.html")
        },
        Err(_) => {
            error!("Cannot remove account: {}", &decoded.user_id); 
            format!("http://127.0.0.1/homepage.html")
        }
    }
}
