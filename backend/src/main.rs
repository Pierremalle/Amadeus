mod error;
mod routes;
mod models;

use std::sync::LazyLock;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

#[macro_use] extern crate rocket;

async fn init() -> Result<(), surrealdb::Error> {
    DB.connect::<Ws>("localhost:5432").await?;

    DB.signin(Root {
        username: "root",
        password: "root",
    })
        .await?;

    DB.use_ns("namespace").use_db("database").await?;

    DB.query(
        "    DEFINE TABLE person SCHEMALESS
        PERMISSIONS FOR
            CREATE, SELECT WHERE $auth,
            FOR UPDATE, DELETE WHERE created_by = $auth;
    DEFINE FIELD name ON TABLE person TYPE string;
    DEFINE FIELD created_by ON TABLE person VALUE $auth READONLY;

    DEFINE INDEX unique_name ON TABLE user FIELDS name UNIQUE;
    DEFINE ACCESS account ON DATABASE TYPE RECORD
	SIGNUP ( CREATE user SET name = $name, pass = crypto::argon2::generate($pass) )
	SIGNIN ( SELECT * FROM user WHERE name = $name AND crypto::argon2::compare(pass, $pass) )
	DURATION FOR TOKEN 15m, FOR SESSION 12h
;",
    )
        .await?;
    Ok(())
}

#[launch]
pub async fn rocket() -> _ {
    init().await.expect("Something went wrong, shutting down");
    rocket::build().mount(
        "/",
        routes![
            routes::create_person,
            routes::read_person,
            routes::update_person,
            routes::delete_person,
            routes::list_people,
            routes::paths,
            routes::make_new_user,
            routes::get_new_token,
            routes::session
        ],
    )
}