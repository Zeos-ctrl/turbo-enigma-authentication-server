mod auth;
mod connections;

use rocket::fs::{FileServer, relative};
use connections::connect::{ReRouter,create_table,add_to_table,remove_from_table,edit_table};

#[macro_use] extern crate rocket;

#[launch]
async fn launch_server() -> _ {
    
    let pool = connections::connect::create_connection().await.unwrap();

    rocket::build()
        .manage(connections::connect::Pool(pool))
        .mount("/connections", routes![create_table,add_to_table,remove_from_table,edit_table])
        .mount("/", FileServer::from(relative!("static")))
        .attach(ReRouter)
    }
