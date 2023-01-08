use rocket::response::{self, Response,Responder};
use rocket::http::{ContentType,Status,};

#[derive(Debug)]
pub struct ApiResponse{
    pub auth_token: String,
    pub status: Status,
}

impl<'r> Responder<'r, 'static> for ApiResponse{
    fn respond_to(self, req: &'r rocket::Request<'_>) -> response::Result<'static> {
        Response::build()
            .status(self.status)
            .header(ContentType::JSON)
            .raw_header("Authorization", self.auth_token.to_string())
            .ok()
    }
}
