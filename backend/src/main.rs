mod error;
mod routes;
mod models;
mod cors;

use std::collections::HashMap;
use std::sync::LazyLock;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use env_file_reader::read_file;
use crate::cors::CORS;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket::data::{Limits, ToByteUnit};

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

#[macro_use] extern crate rocket;

async fn init() -> Result<(), surrealdb::Error> {
    let env_variables = read_file("../.env").unwrap_or(HashMap::from([
        ("SURREALDB_USER".to_string(), "root".to_string()),
        ("SURREALDB_PASS".to_string(), "root".to_string()),
        ("BDD_PORT".to_string(), "5432".to_string()),
        ("BDD_HOST".to_string(), "localhost".to_string()),
    ]));
    let host = &env_variables["BDD_HOST"];
    let port = &env_variables["BDD_PORT"];
    let address = format!("{}:{}", host, port);
    DB.connect::<Ws>(&address).await?;

    DB.signin(Root {
        username: &env_variables["SURREALDB_USER"],
        password: &env_variables["SURREALDB_PASS"],
    })
        .await?;

    DB.use_ns("namespace").use_db("database").await?;

    DB.query(
        "
        DEFINE TABLE song SCHEMALESS
        PERMISSIONS
            FOR CREATE, SELECT WHERE $auth,
            FOR UPDATE, DELETE WHERE created_by = $auth;
        DEFINE FIELD timestamp ON TABLE song TYPE string;
        DEFINE FIELD name ON TABLE song TYPE string;
        DEFINE FIELD bpm ON TABLE song TYPE float;
        DEFINE FIELD duration ON TABLE song TYPE float;
        DEFINE FIELD created_by ON TABLE person VALUE $auth READONLY;

        DEFINE TABLE person SCHEMALESS
        PERMISSIONS
            FOR CREATE, SELECT WHERE $auth,
            FOR UPDATE, DELETE WHERE created_by = $auth;
        DEFINE FIELD first_name ON TABLE person TYPE string;
        DEFINE FIELD last_name ON TABLE person TYPE string;
        DEFINE FIELD email ON TABLE person TYPE string;
        DEFINE FIELD password ON TABLE person TYPE string;
        DEFINE FIELD instruments ON TABLE person TYPE array<string>;
        DEFINE FIELD compositions ON TABLE person TYPE array<record<song>>;
        DEFINE FIELD created_by ON TABLE person VALUE $auth READONLY;

        DEFINE FIELD email ON TABLE user TYPE string ASSERT $value.is_email();
        DEFINE INDEX unique_name ON TABLE user FIELDS email UNIQUE;
        DEFINE ACCESS account ON DATABASE TYPE RECORD
            SIGNUP ( CREATE user SET name = $name, email = $email, pass = crypto::argon2::generate($pass) )
            SIGNIN ( SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(pass, $pass) )
            DURATION FOR TOKEN 15m, FOR SESSION 12h;
        ",
    )
        .await?;
    Ok(())
}

#[launch]
pub async fn rocket() -> _ {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);
    init().await.expect("Something went wrong, shutting down");
    rocket::build().attach(cors.to_cors().unwrap()).mount(
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
            routes::session,
            routes::list_songs,
            routes::create_song,
        ],
    )
}