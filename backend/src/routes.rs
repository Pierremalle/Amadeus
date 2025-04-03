use faker_rand::fr_fr::internet::Email;
use faker_rand::fr_fr::names::FirstName;
use rocket::get;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use surrealdb::opt::auth::Record;
use crate::DB;
use crate::error::Error;
use crate::models::person::{Person, PersonData};
use crate::models::song::{Song, SongData};
use chrono::prelude::*;

const PERSON: &str = "person";
const SONG: &str = "song";

#[get("/")]
pub async fn paths() -> &'static str {
    r#"

-----------------------------------------------------------------------------------------------------------------------------------------
        PATH                |           SAMPLE COMMAND
-----------------------------------------------------------------------------------------------------------------------------------------
/session: See session data  |  curl -X GET    -H "Content-Type: application/json"                          http://localhost:8080/session
                            |
/person/{id}:               |
  Create a person           |  curl -X POST   -H "Content-Type: application/json" -d '{"name":"John Doe"}' http://localhost:8080/person/one
  Get a person              |  curl -X GET    -H "Content-Type: application/json"                          http://localhost:8080/person/one
  Update a person           |  curl -X PUT    -H "Content-Type: application/json" -d '{"name":"Jane Doe"}' http://localhost:8080/person/one
  Delete a person           |  curl -X DELETE -H "Content-Type: application/json"                          http://localhost:8080/person/one
                            |
/people: List all people    |  curl -X GET    -H "Content-Type: application/json"                          http://localhost:8080/people

/song: Create a song        |  curl -X POST   -H "Content-Type: application/json" -d '{"name":"Paint in black", "bpm":4.52}' http://localhost:8080/person/one
/songs get songs            |  curl -X GET    -H "Content-Type: application/json"                          http://localhost:8080/songs

/new_user:  Create a new record user
/new_token: Get instructions for a new token if yours has expired"#
}

#[post("/person/<id>", data = "<person>")]
pub async fn create_person(
    id: String,
    person: Json<PersonData>,
) -> Result<Json<Option<Person>>, Error> {
    let person = DB
        .create((PERSON, &*id))
        .content(person.into_inner())
        .await?;
    Ok(Json(person))
}

#[get("/person/<id>")]
pub async fn read_person(id: String) -> Result<Json<Option<Person>>, Error> {
    let person = DB.select((PERSON, &*id)).await?;
    Ok(Json(person))
}

#[put("/person/<id>", data = "<person>")]
pub async fn update_person(
    id: String,
    person: Json<PersonData>,
) -> Result<Json<Option<Person>>, Error> {
    let person = DB
        .update((PERSON, &*id))
        .content(person.into_inner())
        .await?;
    Ok(Json(person))
}

#[delete("/person/<id>")]
pub async fn delete_person(id: String) -> Result<Json<Option<Person>>, Error> {
    let person = DB.delete((PERSON, &*id)).await?;
    Ok(Json(person))
}

#[get("/people")]
pub async fn list_people() -> Result<Json<Vec<Person>>, Error> {
    let people = DB.select(PERSON).await?;
    Ok(Json(people))
}

#[get("/session")]
pub async fn session() -> Result<Json<String>, Error> {
    let res: Option<String> = DB.query("RETURN <string>$session").await?.take(0)?;

    Ok(Json(res.unwrap_or("No session data found!".into())))
}

#[derive(Serialize, Deserialize)]
struct AuthParams<'a> {
    email: &'a str,
    pass: &'a str,
}

#[get("/new_user")]
pub async fn make_new_user() -> Result<String, Error> {
    let first_name = rand::random::<FirstName>().to_string();
    let last_name = rand::random::<FirstName>().to_string();
    let email = rand::random::<Email>().to_string();
    let pass = rand::random::<FirstName>().to_string();
    let jwt = DB
        .signup(Record {
            access: "account",
            namespace: "namespace",
            database: "database",
            params: AuthParams {
                email: &email,
                pass: &pass,
            },
        })
        .await?
        .into_insecure_token();
    Ok(format!("New user created!\n\nFirst name: {first_name}\nLast name: {last_name}\nPassword: {pass}\nToken: {jwt}\n\nTo log in, use this command:\n\nsurreal sql --namespace namespace --database database --pretty --token \"{jwt}\""))
}

#[get("/new_token")]
pub async fn get_new_token() -> String {
    let command = r#"curl -X POST -H "Accept: application/json" -d '{"ns":"namespace","db":"database","ac":"account","user":"your_username","pass":"your_password"}' http://localhost:8000/signin"#;
    format!("Need a new token? Use this command:\n\n{command}\n\nThen log in with surreal sql --namespace namespace --database database --pretty --token YOUR_TOKEN_HERE")
}

#[get("/songs")]
pub async fn list_songs() -> Result<Json<Vec<Song>>, Error> {
    let songs = DB.select(SONG).await?;
    Ok(Json(songs))
}

#[post("/song", data = "<song>")]
pub async fn create_song(
    song: Json<SongData>,
) -> Result<Json<Option<Song>>, Error> {
    let mut data = song.into_inner();
    data.timestamp = Utc::now().to_string();
    let new_song = DB
        .create(SONG)
        .content(data)
        .await?;
    Ok(Json(new_song))
}