use rocket::response::{Flash,Redirect};
use rocket::http::{Cookie,CookieJar};
use log::info;

use super::super::auth::jwt::JwtToken;

//Request guard implementations for the jwt guard

#[get("/homepage.html")]
pub async fn homepage_accept(_jwt: JwtToken){
}

#[get("/signout")]
pub async fn signout(jar: &CookieJar<'_>) -> Flash<Redirect>{
    info!("User {} logged out", Cookie::named("token"));
    jar.remove(Cookie::named("token"));
    Flash::success(Redirect::to(uri!("/index.html")), "Signed Out")
}
