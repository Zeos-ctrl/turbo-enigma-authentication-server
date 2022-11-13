use rocket::Request;

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("Sorry,'{}' is not a valid path.", req.uri())
}

#[catch(500)]
pub fn server_error() -> String {
    format!("Sorry a server error has occured please try again.")
}

#[catch(422)]
pub fn invalid_form(req: &Request) -> String {
    format!("Sorry but '{}' is an invalid form.", req)
}
