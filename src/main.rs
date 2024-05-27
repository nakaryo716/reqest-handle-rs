use std::{collections::HashMap, sync::{Arc, RwLock}};

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
    routing::{get, post},
    Router,
};
use axum_extra::extract::CookieJar;
use handler::{create_user, get_user, provide_cookie, read_cookie};
use serde::{Deserialize, Serialize};

mod handler;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let app = Router::new()
        .route("/", get(|| async { "Hello".to_string() }))
        .route("/provide-cookie", get(provide_cookie))
        .route("/read-cookie", get(read_cookie))
        .route("/create-user", post(create_user))
        .route("/data-get", get(get_user))
        .with_state(Arc::new(Db::new()));

    axum::serve(listener, app).await.unwrap();
}

// model layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: String,
    name: String,
}

#[derive(Debug, Clone)]
pub struct Db {
    pool: Arc<RwLock<HashMap<String, User>>>,
}

impl Db {
    pub fn new() -> Self {
        Self {
            pool: Arc::default(),
        }
    }
}


#[derive(Debug, Clone)]
struct AuthedBody(String);

#[async_trait]
impl<S> FromRequestParts<S> for AuthedBody
where
    S: Send + Sync,
    Arc<Db>: FromRef<S>,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {

        let state = Arc::from_ref(state);
        {
            let a= state.pool.read().unwrap();
            let a = a.get("cookie-test-value");
            match a {
                Some(user) => println!("{:?}", user),
                None => println!("none"),
            }
        }


        let jar = CookieJar::from_request_parts(parts, &state)
            .await
            .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

        let value = jar.get("cookie-test-name").unwrap().value().to_string();

        Ok(AuthedBody(value))
    }
}
