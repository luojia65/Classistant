use actix_session::Session;
use actix_web::{web, Responder, HttpRequest, HttpResponse};
use rsa::{RSAPrivateKey, PublicKey, PaddingScheme};
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct PublicKeyStruct {
    n: String,
    e: String,
}

lazy_static! {
    static ref AUTH_KEY: RSAPrivateKey = {
        let mut rng = OsRng::new().expect("get secure randomness");
        let bits = 2048;
        RSAPrivateKey::new(&mut rng, bits).expect("generate a key")
    };
    static ref AUTH_KEY_JSON: String = {
        serde_json::to_string(&PublicKeyStruct {
            n: base64::encode(&AUTH_KEY.n().to_bytes_le()),
            e: base64::encode(&AUTH_KEY.e().to_bytes_le()),
        }).expect("serialize as json")
    };
}

pub fn init(_session: Session) -> impl Responder {
    format!("{}", *AUTH_KEY_JSON)
}

#[derive(Serialize, Deserialize)]
struct ValidateResult {
    code: u32,
    description: &'static str
}

pub fn validate(path: web::Path<(String,)>) -> impl Responder {
    // println!("{:?}", AUTH_KEY.encrypt(&mut rand::thread_rng(), PaddingScheme::PKCS1v15, 
    //     &path.0.as_bytes()
    // ));
    let _user_id = &path.0;
    
    HttpResponse::Ok().json(ValidateResult {
        code: 0,
        description: "Login succeeded"
    })
}
