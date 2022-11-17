use sqlx::MySqlPool;
use sqlx::mysql::MySqlPoolOptions;
use rocket::{Request, Response};
use rocket::fairing::{Fairing,Info,Kind};
use rocket::http::{Method, ContentType, Status};
use std::io::Cursor;

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
        .connect("mysql://root:password@mysql/db")
        .await?;
    Ok(pool)
}

//
// Start of tests
//

#[sqlx::test]
async fn create_connection_test(){
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:password@localhost/db")
        .await;
    match pool{
        Ok(_pool) => assert!(true),
        Err(_pool) => assert!(false)
    }
}
