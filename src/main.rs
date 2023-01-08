mod auth;
mod connections;
mod api;
mod catchers;
mod tests;

use rocket::fs::{FileServer, relative};
use env_logger::Builder;
use log::LevelFilter;

use connections::connect::ReRouter;
use api::{login,register,modify,remove,api_guards};
use auth::captcha::gen_captcha;
use auth::otp::gen_qr;
use catchers::catchers::{not_found,server_error,invalid_form};

#[macro_use] extern crate rocket;

#[launch]
async fn launch_server() -> _ {
    //Builder for good looking logs
    let mut builder = Builder::from_default_env();
    builder
        .filter(None, LevelFilter::Info)
        .init();

    //Starts a connection to the database
    let pool = connections::connect::create_connection().await.unwrap();

    //Starts the server and mounts the routes
    rocket::build()
        .manage(connections::connect::Pool(pool))
        .register("/", catchers![not_found,server_error,invalid_form])
        .mount("/api", routes![
               login::login,
               register::register,
               modify::modify,
               remove::remove_account
        ])
        .mount("/", routes![
               api_guards::homepage_accept,
               api_guards::signout,
               gen_captcha,
               gen_qr
        ])
        .mount("/", FileServer::from(relative!("static")))
        .attach(ReRouter)
}
