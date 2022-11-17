use rocket::Request;
use rocket::response::{Flash,Redirect};

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("Sorry,'{}' is not a valid path.", req.uri())
}

#[catch(500)]
pub fn server_error() -> String {
    format!("Sorry a server error has occured please try again.")
}

#[catch(422)]
pub fn invalid_form(req: &Request) -> Flash<Redirect> {
    error!("Sorry but '{:?}' is an invalid form.", req);
    Flash::error(Redirect::to(uri!("./../index.html")),"Invalid form")
}
