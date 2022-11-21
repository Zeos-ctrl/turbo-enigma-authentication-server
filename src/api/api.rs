use super::super::auth::account::{User,NewUser,Login,Verify};
use super::super::auth::jwt::JwtToken;
use super::super::auth::otp::verify_totp;

use totp_rs::Secret;
use sqlx::MySqlPool;
use rocket::form::Form;
use rocket::response::{Flash,Redirect};
use rocket::http::{Cookie,CookieJar};
use rocket::State;
use uuid::Uuid;
use log::{error, info};

//Empty pool struct to be managed by each function
pub struct Pool(pub MySqlPool);

#[post("/login", data = "<login>")]
pub async fn login(pool: &State<Pool>,jar: &CookieJar<'_>, login: Form<Login>) -> Flash<Redirect>{
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
                    jar.add(Cookie::new("token", JwtToken::encode(&user.uuid)));
                    info!("Logged in user: {}", &login.username);
                    Flash::success(Redirect::to(uri!("/homepage")), "Correct credentials")
                }else{
                    error!("Incorrect login credentials for: {}", &login.username);
                    Flash::error(Redirect::to(uri!("/index.html")), "Incorrect credentials")
                }
            },
            Err(_) => Flash::error(Redirect::to(uri!("/index.html")), "Error connecting to database")
        }
    }else{
        error!("Incorrect OTP during login for: {}", &login.username);
        Flash::error(Redirect::to(uri!("/index.html")),"Incorrect OTP")
    }
}

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

#[post("/edit", data ="<newuser>")]
pub async fn edit_account(pool: &State<Pool>, token: JwtToken, newuser: Form<NewUser>) -> Flash<Redirect>{
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

pub async fn _create_table(pool: &State<Pool>){
    let query = sqlx::query(
    r#"
    CREATE TABLE IF NOT EXISTS users (
    uuid varchar(255) PRIMARY KEY NOT NULL,
    username varchar(255) NOT NULL,
    email varchar(255) NOT NULL,
    password varchar(255) NOT NULL,
    phonenumber int,
    seconds int,
    CONSTRAINT users UNIQUE (username,email)
    );"#,
    )
    .execute(&pool.0)
    .await;

    match query {
        Ok(_query) => info!("Created table users!"),
        Err(_) => error!("Cannot create table.")
    }
}

//
//  START OF TESTS
//

#[sqlx::test]
async fn add_user_to_users_test(pool: MySqlPool){
    let id = Uuid::new_v4();
    let _query =sqlx::query!(
        r#"
        INSERT INTO users (uuid,username,email,password,phonenumber,secret)
        VALUES (?,'test','test','test',1,'secret');"#,
        id.to_string()
    )
    .execute(&pool)
    .await;

    match _query {
        Ok(_query) => assert!(true),
        Err(_query) => {
            panic!("ERROR ADDING A USER: {}", _query)
        }
    } 
}

#[sqlx::test]
async fn remove_user_from_users_test(pool: MySqlPool){
    let _query =sqlx::query(
        r#"
        DELETE FROM users
        WHERE uuid = 'test';"#,
    )
    .execute(&pool)
    .await;

    match _query {
        Ok(_query) => assert!(true),
        Err(_query) => {
            panic!("ERROR REMOVING A USER: {}", _query)
        }
    } 
}

#[sqlx::test]
async fn edit_table_test(pool: MySqlPool){
    
    let password: String = String::from("test");
    let username: String = String::from("test");
    //Adds a test user for a correct edit
    sqlx::query!(
        r#"
        INSERT INTO users (uuid,username,email,password,phonenumber,secret)
        VALUES (1,'test','test',?,1,'secret');"#,
        User::hash_password(&password)
        )
        .execute(&pool)
        .await
        .unwrap();

    let _query = sqlx::query!(
        r#"
        UPDATE users
        SET password = ?
        WHERE username = ?
        AND password = ?;"#,
        "newPassword",
        username,
        User::hash_password("password")
        )
        .execute(&pool)
        .await;
    
    match _query {
        Ok(_query) => assert!(true),
        Err(_query) => {
            panic!("ERROR EDITING DATABASE: {}",_query)
        }
    }
}


#[sqlx::test]
async fn create_table_test(pool: MySqlPool){
    let _query =sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
        uuid varchar(255) PRIMARY KEY NOT NULL,
        username varchar(255) NOT NULL,
        email varchar(255) NOT NULL,
        password varchar(255) NOT NULL,
        phonenumber INT,
        secret varchar(255) NOT NULL,
        CONSTRAINT users UNIQUE (username,email)
        );"#,
    )
    .execute(&pool)
    .await;

    match _query {
        Ok(_query) => assert!(true),
        Err(_query) => {
            panic!("ERROR CREATING A DATABASE: {}", _query)
        } 
    }
}
#[sqlx::test]
async fn return_table_test(pool: MySqlPool){
    let _table = sqlx::query!(
        r#"
        SELECT *
        FROM users
        "#
        )
        .fetch_all(&pool)
        .await;

    match _table {
        Ok(_table) => assert!(true),
        Err(_table) => {
            panic!("ERROR READING DATABASE: {}",_table)
        }
    }
}
