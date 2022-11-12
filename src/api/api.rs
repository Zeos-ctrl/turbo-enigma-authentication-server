use super::super::auth::account::{User,NewUser,Login,Verify};
use super::super::auth::jwt::JwtToken;
use super::super::auth::otp::verify_totp;
use totp_rs::Secret;
use sqlx::MySqlPool;
use sqlx::mysql::MySqlPoolOptions;
use rocket::form::Form;
use rocket::response::{Flash,Redirect};
use rocket::http::{Cookie,CookieJar};
use rocket::State;
use uuid::Uuid;

pub struct Pool(pub MySqlPool);

#[post("/login", data = "<login>")]
pub async fn login(pool: &State<Pool>,jar: &CookieJar<'_>, login: Form<Login>) -> Flash<Redirect>{
    let secret = String::from("KRSXG5CTMVRXEZLUKN2XAZLSKNSWG4TFOQ");
    if verify_totp(secret,&login.otp){
        let user = sqlx::query!(
            r#"
            SELECT *
            FROM db
            WHERE username = ?;
            "#,
            &login.username
            )
            .fetch_one(&pool.0)
            .await
            .unwrap();
        if login.verify_password(&user.password){
            jar.add(Cookie::new("token", JwtToken::encode(&user.uuid)));
            Flash::success(Redirect::to(uri!("/homepage")), "Correct credentials")
        }else{
            Flash::error(Redirect::to(uri!("/index.html")), "Incorrect credentials")
        }
    }else{
        Flash::error(Redirect::to(uri!("/index.html")),"Incorrect OTP")
    }
}

#[post("/add", data = "<user>")]
pub async fn register(pool: &State<Pool>,jar: &CookieJar<'_>, user: Form<User>) -> Redirect{
    let captcha = jar.get("captcha").unwrap().to_string();
    let secret = Secret::Encoded("KRSXG5CTMVRXEZLUKN2XAZLSKNSWG4TFOQ".to_string()).to_bytes().unwrap();
    println!("{}",&captcha[8..]);
    if user.captcha == captcha[8..] {
        let id = Uuid::new_v4();
        sqlx::query!(
            r#"
            INSERT INTO db (uuid,username,email,password,phonenumber,secret)
            VALUES (?,?,?,?,?,?);"#,
            id.to_string(),
            &user.username,
            &user.email,
            User::hash_password(&user.password),
            &user.phonenumber,
            secret
        )
        .execute(&pool.0)
        .await
        .unwrap();
        
        Redirect::to(uri!("/index.html"))
    }else{
        Redirect::to(uri!("/index.html"))
    }

}

#[delete("/remove")]
pub async fn remove_account(pool: &State<Pool>, token: JwtToken) {
    sqlx::query!(
        r#"
        DELETE FROM db
        WHERE username = ?;"#,
        &token.user_id
    )
    .execute(&pool.0)
    .await
    .unwrap();
}

#[post("/edit", data ="<newuser>")]
pub async fn edit_account(pool: &State<Pool>, token: JwtToken, newuser: Form<NewUser>){
    sqlx::query!(
        r#"
        UPDATE db
        SET password = ?
        WHERE uuid = ?
        AND password = ?;"#,
        &newuser.new_password,
        &token.user_id,
        User::hash_password(&newuser.old_password)
        )
        .execute(&pool.0)
        .await
        .unwrap();
}

#[get("/create")]
pub async fn create_table(pool: &State<Pool>){
    sqlx::query(
    r#"
    CREATE TABLE IF NOT EXISTS db (
    uuid varchar(255) PRIMARY KEY NOT NULL,
    username varchar(255) NOT NULL,
    email varchar(255) NOT NULL,
    password varchar(255) NOT NULL,
    phonenumber int,
    seconds int,
    CONSTRAINT db UNIQUE (username,email)
    );"#,
    )
    .execute(&pool.0)
    .await
    .unwrap();
}

#[sqlx::test]
async fn add_user_to_db_test(pool: MySqlPool){
    let id = Uuid::new_v4();
    let _query =sqlx::query!(
        r#"
        INSERT INTO db (uuid,username,email,password,phonenumber)
        VALUES (?,'test','test','test',1);"#,
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
async fn remove_user_from_db_test(pool: MySqlPool){
    let _query =sqlx::query(
        r#"
        DELETE FROM db
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
        INSERT INTO db (uuid,username,email,password,phonenumber)
        VALUES (1,'test','test',?,1);"#,
        User::hash_password(&password)
        )
        .execute(&pool)
        .await
        .unwrap();

    let _query = sqlx::query!(
        r#"
        UPDATE db
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
async fn create_connection_test(){
    let _pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:password@localhost/db")
        .await;
    match _pool{
        Ok(_pool) => assert!(true),
        Err(_pool) => assert!(false)
    }
}

#[sqlx::test]
async fn create_table_test(pool: MySqlPool){
    let _query =sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS db (
        uuid varchar(255) PRIMARY KEY NOT NULL,
        username varchar(255) NOT NULL,
        email varchar(255) NOT NULL,
        password varchar(255) NOT NULL,
        phonenumber INT,
        CONSTRAINT db UNIQUE (username,email)
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
        FROM db
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
