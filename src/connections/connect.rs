use super::super::auth::authentication::{User,NewUser};
use sqlx::MySqlPool;
use sqlx::mysql::MySqlPoolOptions;
use rocket::form::Form;
use rocket::{Request, Response,State};
use rocket::fairing::{Fairing,Info,Kind};
use rocket::http::{Method, ContentType, Status};
use rocket::serde::json::Json;
use std::io::Cursor;

pub struct Pool(pub MySqlPool);

pub struct ReRouter;

#[rocket::async_trait]
impl Fairing for ReRouter {
    
    fn info(&self) -> Info {
        Info {
            name: "GET rerouter",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>,response: &mut Response<'r>) {
        if request.method() == Method::Get &&
            response.status() == Status::NotFound {
                let body = format!("URL does not exist");
                response.set_status(Status::Ok);
                response.set_header(ContentType::Plain);
                response.set_sized_body(body.len(),Cursor::new(body));
            }
        return
    }
}

pub async fn create_connection() -> Result<MySqlPool, sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:password@localhost/db")
        .await?;
    Ok(pool)
}


#[get("/create")]
pub async fn create_table(pool: &State<Pool>){
    sqlx::query(
    r#"
    CREATE TABLE IF NOT EXISTS db (
    uuid int PRIMARY KEY NOT NULL,
    username varchar(255) NOT NULL,
    email varchar(255) NOT NULL,
    password varchar(255) NOT NULL,
    phonenumber int,
    CONSTRAINT db UNIQUE (username,email)
    );"#,
    )
    .execute(&pool.0)
    .await
    .unwrap();
}

#[post("/add", format = "json", data = "<user>")]
pub async fn add_to_table(pool: &State<Pool>, user: Json<User>){
    sqlx::query!(
        r#"
        INSERT INTO db (uuid,username,email,password,phonenumber)
        VALUES (?,?,?,?,?);"#,
        1,
        &user.username,
        &user.email,
        User::hash_password(&user.password),
        &user.phonenumber
    )
    .execute(&pool.0)
    .await
    .unwrap();
}

#[delete("/remove/<user>")]
pub async fn remove_from_table(pool: &State<Pool>, user: &str) {
    let username = user.to_string();
    sqlx::query!(
        r#"
        DELETE FROM db
        WHERE username = ?;"#,
        username
    )
    .execute(&pool.0)
    .await
    .unwrap();
}

#[post("/edit", format = "json", data ="<newuser>")]
pub async fn edit_table(pool: &State<Pool>,newuser: Json<NewUser>){
    sqlx::query!(
        r#"
        UPDATE db
        SET password = ?
        WHERE username = ?
        AND password = ?;"#,
        &newuser.new_password,
        &newuser.username,
        User::hash_password(&newuser.old_password)
        )
        .execute(&pool.0)
        .await
        .unwrap();
}

//ONLY USED FOR DEVELOPMENT TAKE AWAY THE POST FOR FINAL SUBMISSION
#[post("/$gOlC££SssXTyuXDE7PydCDx74zGKF")]
pub async fn return_table(pool: &State<Pool>){
    let table = sqlx::query!(
        r#"
        SELECT *
        FROM db
        "#
        )
        .fetch_all(&pool.0)
        .await
        .unwrap();

    for entry in table {
        println!(
            "| {:?} | {:?} | {:?} | {:?} | {:?} |",
            &entry.uuid,
            &entry.username,
            &entry.email,
            &entry.password,
            &entry.phonenumber
            );
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
        uuid int PRIMARY KEY NOT NULL,
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
async fn add_user_to_db_test(pool: MySqlPool){
    let _query =sqlx::query(
        r#"
        INSERT INTO db (uuid,username,email,password,phonenumber)
        VALUES (1,'test','test','test',1);"#,
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
