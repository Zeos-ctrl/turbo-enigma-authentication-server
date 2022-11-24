use serde::Deserialize;
use serde::Serialize;

//File containing structs for all the forms and implementations for those structs

#[derive(FromForm,Serialize,Deserialize)]
pub struct User{
    //Defines the values of the submited form and validates the inputs
    #[field(validate = contains('@'))]
    #[field(validate = len(1..30))]
    pub email: String,
    pub phonenumber: String,
    #[field(validate = len(3..15))]
    pub username: String,
    #[field(validate = len(3..20))]
    pub password: String,
    pub captcha: String
}

#[derive(FromForm,Serialize,Deserialize)]
pub struct NewUser{
    #[field(validate = len(2..20))]
    pub new_password: String,
}

#[derive(FromForm,Serialize,Deserialize)]
pub struct Login{
    #[field(validate = len(3..15))]
    pub username: String,
    #[field(validate = len(3..20))]
    pub password: String,
    pub otp: String
}

pub trait Verify {
    fn verify_password(&self, password: &str) -> bool;
}

impl Verify for Login {
    fn verify_password(&self, password: &str) -> bool{
        bcrypt::verify(&self.password, &password).unwrap()
    }
}

impl Verify for User {
    fn verify_password(&self, password: &str) -> bool{
        bcrypt::verify(&self.password, &password).unwrap()
    }
}

impl User{
    pub fn hash_password(unhashed_password: &str) -> String{
        let cost: u32 = 10;
        let hashed_password = bcrypt::hash(unhashed_password,cost).unwrap();
        hashed_password
    }
}

//
// Start of tests
//

#[test]
fn hash_and_verify_password(){
    let unhashed_password = String::from("test");
    let cost: u32 = 10;
    let hashed_password = bcrypt::hash(unhashed_password,cost).unwrap();
    println!("{}",hashed_password);
    let verify = bcrypt::verify("test", &hashed_password).unwrap();
    
    assert_eq!(verify,true);
}
