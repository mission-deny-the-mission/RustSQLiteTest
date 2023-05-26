#[macro_use] extern crate actix_web;

use rusqlite::{Connection};
use std::path::PathBuf;
use askama::Template;
use actix_web::{HttpServer, App, Responder, HttpResponse};
use actix_files::NamedFile;

struct Person {
    id: i64,
    name: String,
    date_of_birth: String,
    phone_number: String,
}

#[derive(Template)]
#[template(path = "people.html")]
struct PeopleTemplate <'a> {
    people: &'a Vec<Person>,
}

#[get("/people")]
async fn people() -> actix_web::Result<impl Responder> {
    let conn = Connection::open("people.db").expect("msg");
    let query = "SELECT * FROM People";
    let mut statement = conn.prepare(query).expect("msg");
    let people = statement.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            date_of_birth: row.get(2)?,
            phone_number: row.get(3)?,
        })
    }).expect("msg");
    
    let mut people_vec: Vec<Person> = Vec::new();
    for person_i in people {
        people_vec.push(person_i.unwrap())
    }
    let template = PeopleTemplate {people: &people_vec};
    let html = template.render().unwrap();

    Ok(HttpResponse::Ok().body(html))
}

#[get("/")]
async fn index() -> actix_web::Result<impl Responder> {
    let path: PathBuf = "./static/index.html".parse().unwrap();
    Ok(NamedFile::open(path))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
        .service(index)
        .service(people)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}