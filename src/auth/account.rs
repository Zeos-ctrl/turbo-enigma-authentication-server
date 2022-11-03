use serde::Deserialize;
use serde::Serialize;

#[derive(FromForm,Serialize,Deserialize)]
pub struct User{
    pub email: String,
    pub phonenumber: String,
    pub username: String,
    pub password: String
}

#[derive(FromForm,Serialize,Deserialize)]
pub struct NewUser{
    pub new_password: String,
    pub username: String,
    pub old_password:String

}

#[derive(FromForm,Serialize,Deserialize)]
pub struct Login{
    pub username: String,
    pub password: String
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

#[test]
fn hash_and_verify_password(){
    let unhashed_password = String::from("test");
    let cost: u32 = 10;
    let hashed_password = bcrypt::hash(unhashed_password,cost).unwrap();
    println!("{}",hashed_password);
    let verify = bcrypt::verify("test", &hashed_password).unwrap();
    
    assert_eq!(verify,true);
}
