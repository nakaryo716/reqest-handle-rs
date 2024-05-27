use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::{cookie::Cookie, CookieJar};

use crate::{AuthedBody, CreateUser, Db, User};

pub async fn provide_cookie(jar: CookieJar) -> impl IntoResponse {
    let cookie = Cookie::new("cookie-test-name", "cookie-test-value");
    let res = jar.add(cookie);

    (StatusCode::OK, res)
}

pub async fn read_cookie(jar: CookieJar) -> impl IntoResponse {
    let cookie_value = jar.get("cookie-test-name");

    match cookie_value {
        Some(value) => {
            let text = format!("we have cookie-value: {}", value);
            text.into_response()
        }
        None => "none".to_string().into_response(),
    }
}

pub async fn create_user(
    AuthedBody(text): AuthedBody,
    State(db): State<Arc<Db>>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let user = User {
        id: text.clone(),
        name: payload.name,
    };

    db.pool.write().unwrap().insert(text.clone(), user.clone());

    (StatusCode::OK, Json(user))
}

pub async fn get_user(
    AuthedBody(text): AuthedBody,
    State(db): State<Arc<Db>>, 
) -> impl IntoResponse {
    println!("debug get-user: {}", text.clone());

    let user = db.pool.read().unwrap().get(&text).cloned();

    println!("{:?}", user);

    match user {
        Some(user) => (StatusCode::OK, Json(user)).into_response(),
        None => (StatusCode::BAD_GATEWAY).into_response(),
    }
}

// pub async fn insert(AuthedBody(mut text): AuthedBody, State(mut db): State<Db>, Json(payload): Json<CreateUser>) -> impl IntoResponse {
//     text.push_str("1111111");
//     let user = User {
//         id: text,
//         name: payload.name,
//     };

//    db.pool.insert("hello".to_string(), user);
// }
