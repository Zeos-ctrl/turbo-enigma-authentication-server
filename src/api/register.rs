use super::super::auth::account::User;
use super::super::connections::connect::Pool;

use rocket::State;
use totp_rs::Secret;
use rocket::form::Form;
use rocket::response::{Flash,Redirect};
use rocket::http::CookieJar;
use uuid::Uuid;
use log::{error, info};

use super::response::ApiResponse;

#[post("/add", data = "<user>")]
pub async fn register(pool: &State<Pool>,jar: &CookieJar<'_>, user: Form<User>) -> Flash<Redirect>{

    //Gets the captcha cookie and sees if its the same as the user input
    let captcha = jar.get("captcha").unwrap().to_string();
    let secret = Secret::Encoded("KRSXG5CTMVRXEZLUKN2XAZLSKNSWG4TFOQ".to_string()).to_bytes().unwrap();
    if user.captcha == captcha[8..] {
        let id = Uuid::new_v4();
        let query = sqlx::query!(
            r#"
            INSERT INTO users (uuid,username,email,password,phonenumber,secret)
            VALUES (?,?,?,?,?,?);"#,
            id.to_string(),
            &user.username,
            &user.email,
            User::hash_password(&user.password),
            &user.phonenumber,
            secret
        )
        .execute(&pool.0)
        .await;

        match query {
            Ok(_query) => {
                info!("Registered new user: {}", &user.username); 
                Flash::success(Redirect::to(uri!("/index.html")),"Successful register")
            },
            Err(_) =>{
                error!("Couldn't register user: {}", &user.username);
                Flash::error(Redirect::to(uri!("/index.html")),"Cannot register user")
            } 
        }
    }else{
        error!("Incorrect captcha solution");
        Flash::error(Redirect::to(uri!("/index.html")),"Incorrect captcha")
    }
}
