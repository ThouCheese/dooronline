use db::get_connection;
use models::User;
use schema;
use views::index::open_door;
use rocket::{response::Failure, http::Status, };
use rocket_contrib::Json;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
use crypto::validate_user_token;
use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

#[derive(Serialize, Deserialize)]
struct LoginJson {
     username: String,
     password: String,
}

struct RequestToken(String);

impl<'a, 'r> FromRequest<'a, 'r> for RequestToken {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<RequestToken, ()> {
        println!("reached1");
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        println!("keys is {:?}", keys);
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }
        let key = keys[0];
        if key[0..6] != *"Bearer" {
            return Outcome::Failure((Status::BadRequest, ()));
        }
        return Outcome::Success(RequestToken(key[7..].to_string()))
    }
}

#[post("/login", data="<form>")]
fn login(form: Json<LoginJson>) -> Result<Json<LoginResponse>, Failure> {
    let user: User = schema::user::table
        .filter(schema::user::username.eq(&form.username))
        .get_result(&get_connection())
        .map_err(|_| Failure(Status::BadRequest))?;
    if user.validate_password(&form.password) {
        Ok(Json(LoginResponse{token: user.create_jwt().unwrap()}))
    } else {
        Err(Failure(Status::Unauthorized))
    }

}

#[get("/open")]
fn open(token: RequestToken) -> Result<(), Failure> {
    let user = validate_user_token(&token.0)
        .ok_or_else(|| Failure(Status::BadRequest))?;

    open_door(&user)?;
    Ok(())
}