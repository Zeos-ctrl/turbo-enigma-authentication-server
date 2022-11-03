mod auth;
mod connections;
mod api;

use rocket::fs::{FileServer, relative};
use connections::connect::{ReRouter,create_table,return_table,drop};
use api::api::{login,register,remove_account,edit_account};

#[macro_use] extern crate rocket;

#[launch]
async fn launch_server() -> _ {
    
    let pool = connections::connect::create_connection().await.unwrap();

    rocket::build()
        .manage(connections::connect::Pool(pool))
        .mount("/connections", routes![create_table,return_table,drop])
        .mount("/api", routes![login,register,remove_account,edit_account])
        .mount("/", FileServer::from(relative!("static")))
        .attach(ReRouter)
    }
