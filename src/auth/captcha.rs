use captcha_rs::CaptchaBuilder;
use rocket::http::{CookieJar, Cookie};

#[get("/gen_captcha")]
pub fn gen_captcha(jar: &CookieJar<'_>){
    let captcha = CaptchaBuilder::new()
        .length(5)
        .width(220)
        .height(110)
        .dark_mode(false)
        .complexity(1)
        .build();

    let image_string: String = captcha.base_img[22..].to_string();
    let image_data = base64::decode(image_string).unwrap();

    let img = image::load_from_memory_with_format(&image_data,image::ImageFormat::Png).unwrap();
    img.save("./../static/captcha/captcha.png").unwrap();

    jar.add(Cookie::new("captcha", captcha.text));
}

#[test]
fn test_gen_captcha(){
    let captcha = CaptchaBuilder::new()
        .length(5)
        .width(220)
        .height(110)
        .dark_mode(false)
        .complexity(1)
        .build();

    let image_string: String = captcha.base_img[22..].to_string();
    let image_data = base64::decode(image_string).unwrap();

    let img = image::load_from_memory_with_format(&image_data,image::ImageFormat::Png).unwrap();
    img.save("./static/captcha/captcha.png").unwrap();

}
