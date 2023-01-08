#[cfg(test)]
mod tests{
    /*
     * Test library for major database tests
     */

    use sqlx::MySqlPool;
    use sqlx::mysql::MySqlPoolOptions;
    use uuid::Uuid;
    use crate::auth::account::User;

    #[sqlx::test]
    async fn add_user_to_users_test(pool: MySqlPool){
        let id = Uuid::new_v4();
        let _query =sqlx::query!(
            r#"
            INSERT INTO users (uuid,username,email,password,phonenumber,secret)
            VALUES (?,'test','test','test',1,'secret');"#,
            id.to_string()
            )
            .execute(&pool)
            .await;

        match _query {
            Ok(_query) => assert!(true),
            Err(_query) => {
                panic!("ERROR ADDING A USER: {}", _query)
            }
        } 
    }

    #[sqlx::test]
    async fn remove_user_from_users_test(pool: MySqlPool){
        let _query =sqlx::query(
            r#"
            DELETE FROM users
            WHERE uuid = 'test';"#,
            )
            .execute(&pool)
            .await;

        match _query {
            Ok(_query) => assert!(true),
            Err(_query) => {
                panic!("ERROR REMOVING A USER: {}", _query)
            }
        } 
    }

    #[sqlx::test]
    async fn edit_table_test(pool: MySqlPool){

        let password: String = String::from("test");
        let username: String = String::from("test");
        //Adds a test user for a correct edit
        sqlx::query!(
            r#"
            INSERT INTO users (uuid,username,email,password,phonenumber,secret)
            VALUES (1,'test','test',?,1,'secret');"#,
            User::hash_password(&password)
            )
            .execute(&pool)
            .await
            .unwrap();

        let _query = sqlx::query!(
            r#"
            UPDATE users
            SET password = ?
            WHERE username = ?
            AND password = ?;"#,
            "newPassword",
            username,
            User::hash_password("password")
            )
            .execute(&pool)
            .await;

        match _query {
            Ok(_query) => assert!(true),
            Err(_query) => {
                panic!("ERROR EDITING DATABASE: {}",_query)
            }
        }
    }


    #[sqlx::test]
    async fn create_table_test(pool: MySqlPool){
        let _query =sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                uuid varchar(255) PRIMARY KEY NOT NULL,
                username varchar(255) NOT NULL,
                email varchar(255) NOT NULL,
                password varchar(255) NOT NULL,
                phonenumber INT,
                secret varchar(255) NOT NULL,
                CONSTRAINT users UNIQUE (username,email)
                );"#,
                )
            .execute(&pool)
            .await;

        match _query {
            Ok(_query) => assert!(true),
            Err(_query) => {
                panic!("ERROR CREATING A DATABASE: {}", _query)
            } 
        }
    }
    #[sqlx::test]
    async fn return_table_test(pool: MySqlPool){
        let _table = sqlx::query!(
            r#"
            SELECT *
            FROM users
            "#
            )
            .fetch_all(&pool)
            .await;

        match _table {
            Ok(_table) => assert!(true),
            Err(_table) => {
                panic!("ERROR READING DATABASE: {}",_table)
            }
        }
    }

    /*
     * Auth tests
     */

    use sha2::Sha256;
    use hmac::{Hmac, NewMac};
    use std::collections::BTreeMap;
    use jwt::{VerifyWithKey, SignWithKey};
    use totp_rs::{Algorithm, TOTP, Secret};
    use captcha_rs::CaptchaBuilder;

    #[test]
    fn hash_and_verify_password(){
        let unhashed_password = String::from("test");
        let cost: u32 = 10;
        let hashed_password = bcrypt::hash(unhashed_password,cost).unwrap();
        println!("{}",hashed_password);
        let verify = bcrypt::verify("test", &hashed_password).unwrap();

        assert_eq!(verify,true);
    }

    #[test]
    fn test_gen_captcha(){
        let captcha = CaptchaBuilder::new()
            .length(5)
            .width(220)
            .height(110)
            .dark_mode(true)
            .complexity(1)
            .build();

        let image_string: String = captcha.base_img[22..].to_string();
        let image_data = base64::decode(image_string).unwrap();

        let img = image::load_from_memory_with_format(&image_data,image::ImageFormat::Png).unwrap();
        img.save("./static/captcha/captcha.png").unwrap();

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

    #[test]
    fn gen_qr_test() {
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
                let image_data = base64::decode(code).unwrap();
                let img = image::load_from_memory_with_format(&image_data,image::ImageFormat::Png).unwrap();
                img.save("./static/qr/qr.png").unwrap();
                assert!(true)
            },
            Err(e) => panic!("ERROR GENERATING QR: {}", e)
        }
    }

    /*
     * Database tests
     */

    #[sqlx::test]
    async fn create_connection_test(){
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect("mysql://root:password@localhost/db")
            .await;
        match pool{
            Ok(_pool) => assert!(true),
            Err(_pool) => assert!(false)
        }
    }
}
