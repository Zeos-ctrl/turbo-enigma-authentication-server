use jwt::{Header, Token, VerifyWithKey};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;
use std::result::Result;
use hmac::{Hmac, NewMac};
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use rocket::outcome::Outcome;

#[derive(Debug)]
pub struct JwtToken {
    pub user_id: String,
    pub body: String
}

#[derive(Debug)]
pub enum ApiKeyError{
    //BadCount,
    //Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JwtToken{
    type Error = ApiKeyError;

/*    async fn _from_request(req: &'r Request<'_>) -> request::Outcome<Self,Self::Error> {
        let keys: Vec<_> = req.headers().get("token").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            1 => {
                let token = JwtToken::decode(String::from(keys[0].to_string()));
                match token {
                    Ok(token) => Outcome::Success(token),
                    Err(_) => Outcome::Failure((Status::Unauthorized, ApiKeyError::Invalid))
                }
            },
            _ => Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount)),
        }
    }
*/    
    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self,Self::Error> {
        let token = req.cookies().get("token").unwrap();
        let token_value = token.value().to_string();

        let decoded = JwtToken::decode(token_value);

        match decoded {
            Ok(decoded) => Outcome::Success(decoded),
            Err(_) => Outcome::Failure((Status::Unauthorized, ApiKeyError::Invalid))
        }
    }
}
impl JwtToken {

    pub fn encode(user_id: &String) -> String{
        let secret_key: String = String::from("secret");
        let key: Hmac<Sha256> = Hmac::new_varkey(
            &secret_key.as_bytes()
            )
            .unwrap();
        let mut claims = BTreeMap::new();
        claims.insert("user_id", user_id);
        let token_str = claims.sign_with_key(&key).unwrap();
        return String::from(token_str)
    }

    pub fn decode(webtoken: String) -> Result<JwtToken, &'static str>{
        let secret_key: String = String::from("secret");
        let key: Hmac<Sha256> = Hmac::new_varkey(
            &secret_key.as_bytes()
            )
            .unwrap();
        let token_str: &str = webtoken.as_str();
        let token: Result<Token<Header, BTreeMap<String, String>, _ > ,jwt::Error> = 
            token_str.verify_with_key(&key);
        match token {
            Ok(token) => Ok( JwtToken {
                user_id: token.claims() ["user_id"].to_string(),
                body: webtoken}),
            Err(_) => Err("Couldnt Decode token")
            }
        }
    }

#[test]
fn encode_and_decode_from_correct_token(){
    let message: String = String::from("test");
    let key: Hmac<Sha256> = Hmac::new_varkey(&message.as_bytes()).unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub","test");
    let token_str = claims.sign_with_key(&key).unwrap();

    let decode_claims: BTreeMap<String,String> = token_str.verify_with_key(&key).unwrap();
    assert_eq!(decode_claims["sub"],"test");

}

#[test]
fn encode_and_decode_from_incorrect_token(){
    let message: String =String::from("test");
    let key: Hmac<Sha256> = Hmac::new_varkey(&message.as_bytes()).unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("user","test");
    let token_str = claims.sign_with_key(&key).unwrap();

    let decode_claims: BTreeMap<String,String> = token_str.verify_with_key(&key).unwrap();
    assert_ne!(decode_claims["user"],"incorrect token");
}
