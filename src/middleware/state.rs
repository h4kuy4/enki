use std::sync::Arc;

use axum::Extension;
use jwt_auth::Jwt;
use sea_orm::DatabaseConnection;

use crate::model::Account;

pub struct State {
    pub conn: DatabaseConnection,
    pub jwt: Jwt,
    pub account: Account,
}

impl State {
    pub fn new(conn: DatabaseConnection, jwt: Jwt, account: Account) -> Extension<Arc<State>> {
        Extension(Arc::new(State { conn, jwt, account }))
    }
}
