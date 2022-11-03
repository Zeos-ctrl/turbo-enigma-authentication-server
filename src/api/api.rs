use super::super::auth::account::{User,NewUser,Login,Verify};
use sqlx::MySqlPool;
use rocket::form::Form;
use rocket::response::Redirect;
use rocket::State;
use uuid::Uuid;

pub struct Pool(pub MySqlPool);

#[post("/login", data = "<login>")]
pub async fn login(pool: &State<Pool>, login: Form<Login>) -> Redirect{
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
        Redirect::to(uri!("/homepage.html"))
    }else{
        Redirect::to(uri!("/index.html"))
    }
}

#[post("/add", data = "<user>")]
pub async fn register(pool: &State<Pool>, user: Form<User>) -> Redirect{
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO db (uuid,username,email,password,phonenumber)
        VALUES (?,?,?,?,?);"#,
        id.to_string(),
        &user.username,
        &user.email,
        User::hash_password(&user.password),
        &user.phonenumber
    )
    .execute(&pool.0)
    .await
    .unwrap();
    
    Redirect::to(uri!("/index.html"))

}

#[delete("/remove/<user>")]
pub async fn remove_account(pool: &State<Pool>, user: &str) {
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

#[post("/edit", data ="<newuser>")]
pub async fn edit_account(pool: &State<Pool>,newuser: Form<NewUser>){
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

