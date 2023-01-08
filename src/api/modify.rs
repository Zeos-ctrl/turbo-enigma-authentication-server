use super::super::auth::account::{User,NewUser};
use super::super::auth::jwt::JwtToken;
use super::super::connections::connect::Pool;

use rocket::form::Form;
use rocket::response::{Flash,Redirect};
use rocket::State;
use log::{error, info};

use super::response::ApiResponse;

#[post("/edit", data ="<newuser>")]
pub async fn modify(pool: &State<Pool>, token: JwtToken, newuser: Form<NewUser>) -> Flash<Redirect>{
    let query = sqlx::query!(
        r#"
        UPDATE users
        SET password = ?
        WHERE uuid = ?;"#,
        User::hash_password(&newuser.new_password),
        &token.user_id
        )
        .execute(&pool.0)
        .await;

    match query {
        Ok(_query) => {
            info!("Editing account: {}", &token.user_id);
            Flash::success(Redirect::to(uri!("/homepage.html")),"Successfully changed password")
        },
        Err(_) => {
            error!("Cannot edit account: {}", &token.user_id);
            Flash::error(Redirect::to(uri!("/homepage.html")),"Error changing password")
        }
    }
}
