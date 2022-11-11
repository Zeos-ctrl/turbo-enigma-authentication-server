use std::time::{SystemTime, UNIX_EPOCH};
use totp_lite::{totp, Sha512};
use lettre::message::Message;
use lettre::{Transport, SmtpTransport};
use lettre::transport::smtp::authentication::Credentials;
use dotenv::dotenv;
use std::env;

fn get_creds() -> (String,String){
    dotenv().ok();

    let sender = match env::var("EMAIL_USERNAME"){
        Ok(val) => val,
        Err(_) => panic!("EMAIL ACCOUNT not found in enviroment variables"),
    };

    let password = match env::var("EMAIL_PASSWORD"){
        Ok(val) => val,
        Err(_) => panic!("EMAIL PASSWORD not found in enviroment variables"),
    };

    (sender,password)
}

pub fn gen_token(email: &String) -> u64{
    let (sender,password) = get_creds();

    let secret: &[u8] = b"secret";
    let seconds: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    let result: String = totp::<Sha512>(secret,seconds);

    let email = Message::builder()
        .from(format!("EMAIL <{}>",sender).parse().unwrap())
        .to(format!("USER <{}>",email).parse().unwrap())
        .subject("OTP")
        .body(result)
        .unwrap();
    
    let creds = Credentials::new(sender,password);

    let mailer = SmtpTransport::relay("stmp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully"),
        Err(e) => panic!("Couldnt send email: {:?} :(",e),
    }
     
    seconds

}

pub fn verify_token(token: String, timestamp: u64) -> bool{
    
    let secret: &[u8] = b"secret";
    
    if token == totp::<Sha512>(secret, timestamp) {
        true
    }else{
        false
    }
}

#[test]
fn gen_and_verify_proper_token(){
    let secret: &[u8] = b"secret";
    let seconds: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    let result: String = totp::<Sha512>(secret,seconds);

    assert_eq!(result, totp::<Sha512>(secret,seconds))
}

#[test]
fn gen_and_verify_bad_token(){
    let secret: &[u8] = b"secret";
    let seconds: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    assert_ne!(String::from("BadToken"), totp::<Sha512>(secret,seconds))
}

#[test]
fn send_email(){
    let (sender, password) = get_creds();
    println!("{:?}\n{:?}",&sender,&password);

    let receiver = String::from("swimconnor4@gmail.com");

    let secret: &[u8] = b"secret";
    let seconds: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    let result: String = totp::<Sha512>(secret,seconds);

    let email = Message::builder()
        .from(format!("EMAIL <{}>",sender).parse().unwrap())
        .to(format!("USER <{}>",receiver).parse().unwrap())
        .subject("OTP")
        .body(result)
        .unwrap();

    let creds = Credentials::new(sender,password);

    let mailer = SmtpTransport::relay("smtp.mail.yahoo.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => assert!(true),
        Err(e) => panic!("Couldnt send email: {:?} :(",e),
    }
}
