mod auth;
mod connections;

use rocket::fs::{FileServer, relative};
use connections::connect::{ReRouter,create_table,register,remove_from_table,edit_table,return_table, login,drop};

#[macro_use] extern crate rocket;

#[launch]
async fn launch_server() -> _ {
    
    let pool = connections::connect::create_connection().await.unwrap();

    rocket::build()
        .manage(connections::connect::Pool(pool))
        .mount("/connections", routes![create_table,register,remove_from_table,edit_table,return_table,login,drop])
        .mount("/", FileServer::from(relative!("static")))
        .attach(ReRouter)
    }
