use totp_rs::{Algorithm, TOTP, Secret};

//Generates a qr code and returns the image as base64 text to be embedded
//in a img html tag
#[get("/gen_qr")]
pub async fn gen_qr() -> String{
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        Secret::Encoded("KRSXG5CTMVRXEZLUKN2XAZLSKNSWG4TFOQ".to_string()).to_bytes().unwrap(),
        Some("Stinky".to_string()),
        "Authentication@Service.com".to_string(),
    ).unwrap();

    let code = totp.get_qr();

    match code {
        Ok(code) => {
            let qr = format!("{}{}","data:image/png;base64,",code);
            qr
        },
        Err(e) => panic!("ERROR GENERATING QR: {}", e)
    }
}

//Verifys the otp against a secret
pub fn verify_totp(secret: String, token: &str) -> bool{
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        Secret::Encoded(secret).to_bytes().unwrap(),
        Some("Stinky".to_string()),
        "Authentication@Service.com".to_string(),
        ).unwrap();

    totp.check_current(token).unwrap()
}
