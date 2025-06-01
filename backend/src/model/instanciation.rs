use crate::error::error;
use std::sync::LazyLock;
use surrealdb::{
    Surreal,
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
};

pub async fn create_db() -> Result<Surreal<Client>, error::Error> {
    static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

    let _ = DB
        .connect::<Ws>("localhost:8000")
        .await
        .map_err(|e| error::Error::from(e));

    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    let _ = DB
        .use_ns("test")
        .use_db("test")
        .await
        .map_err(|e| error::Error::from(e));

    let _ = DB
        .query(
            "
    DEFINE TABLE IF NOT EXISTS person SCHEMALESS
        PERMISSIONS FOR 
            CREATE, SELECT WHERE $auth,
            FOR UPDATE, DELETE WHERE created_by = $auth;
    DEFINE FIELD IF NOT EXISTS name ON TABLE person TYPE string;
    DEFINE FIELD IF NOT EXISTS created_by ON TABLE person VALUE $auth READONLY;

    DEFINE INDEX IF NOT EXISTS unique_name ON TABLE user FIELDS name UNIQUE;
    DEFINE ACCESS IF NOT EXISTS account ON DATABASE TYPE RECORD
	SIGNUP ( CREATE user SET name = $name, pass = crypto::argon2::generate($pass) )
	SIGNIN ( SELECT * FROM user WHERE name = $name AND crypto::argon2::compare(pass, $pass) )
	DURATION FOR TOKEN 15m, FOR SESSION 12h
;",
        )
        .await
        .map_err(|e| error::Error::from(e));

    Ok(DB.clone())
}
