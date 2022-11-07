use rocket::response::{Flash,Redirect};
use rocket::http::{Cookie,CookieJar};
use super::super::auth::authentication::JwtToken;

#[get("/homepage")]
pub async fn homepage_accept(_jwt: JwtToken) -> Redirect{
    Redirect::to(uri!("/homepage.html"))
}

#[get("/signout")]
pub async fn signout(jar: &CookieJar<'_>) -> Flash<Redirect>{
    jar.remove(Cookie::named("token"));
    Flash::success(Redirect::to(uri!("/index.html")), "Signed Out")
}
