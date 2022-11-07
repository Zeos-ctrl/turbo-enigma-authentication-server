mod auth;
mod connections;
mod api;

use rocket::fs::{FileServer, relative};
use connections::connect::ReRouter;
use api::api::{login,register,remove_account,edit_account,create_table};
use api::api_guards::{homepage_accept,signout};

#[macro_use] extern crate rocket;

#[launch]
async fn launch_server() -> _ {
    
    let pool = connections::connect::create_connection().await.unwrap();

    rocket::build()
        .manage(api::api::Pool(pool))
        .mount("/api", routes![login,register,remove_account,edit_account,create_table])
        .mount("/", routes![homepage_accept,signout])
        .mount("/", FileServer::from(relative!("static")))
        .attach(ReRouter)
    }
