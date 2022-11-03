use sqlx::MySqlPool;
use sqlx::mysql::MySqlPoolOptions;
use rocket::{Request, Response,State};
use rocket::fairing::{Fairing,Info,Kind};
use rocket::http::{Method, ContentType, Status};
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
    uuid varchar(255) PRIMARY KEY NOT NULL,
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

//ONLY USED FOR DEVELOPMENT TAKE AWAY THE POST FOR FINAL SUBMISSION
#[post("/$gOlC££SssXTyuXDE7PydCDx74zGKF")]
pub async fn return_table(pool: &State<Pool>){
    let table = sqlx::query!(
        r#"
        SELECT *
        FROM db
        WHERE username = "test";
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

//DELETE AFTER THE PROJECT IS DONE JUST USED FOR TESTS
#[post("/drop")]
pub async fn drop(pool: &State<Pool>){
    sqlx::query!(
        r#"
        DROP TABLE db;
        "#
        )
        .fetch_all(&pool.0)
        .await
        .unwrap();
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
