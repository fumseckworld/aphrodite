//!
//! # Aphrodite
//!
//! > API in order to manage database
//!
#![allow(clippy::multiple_crate_versions)]

use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{get, main, App, HttpResponse, HttpServer, Responder};
use clap::{Arg, ArgMatches, Command};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{MysqlConnection, PgConnection, SqliteConnection};
use std::io::{Error, ErrorKind};

#[get("/")]
async fn mysql_welcome() -> impl Responder {
    HttpResponse::Ok().body("Welcome on mysql")
}

#[get("/")]
async fn pgsql_welcome() -> impl Responder {
    HttpResponse::Ok().body("Welcome on postgres")
}
#[get("/")]
async fn sqlite_welcome() -> impl Responder {
    HttpResponse::Ok().body("Welcome on sqlite")
}

#[doc = "The command options"]
fn aphrodite() -> ArgMatches {
    Command::new("aphrodite")
        .bin_name("aphrodite")
        .disable_help_flag(true)
        .arg(Arg::new("type").long("type").required(true).short('t'))
        .arg(
            Arg::new("database")
                .short('d')
                .long("database")
                .required(true),
        )
        .arg(Arg::new("user").short('u').long("user").required(false))
        .arg(
            Arg::new("password")
                .short('p')
                .long("password")
                .required(false),
        )
        .arg(Arg::new("host").short('h').long("host").required(false))
        .get_matches()
}
#[doc = "The api entry"]
#[main]
async fn main() -> std::io::Result<()> {
    let app = aphrodite();
    if let Some(t) = app.get_one::<String>("type") {
        if t.eq("postgres") {
            if let Ok(s) = run_postgres(&app) {
                return s.await;
            }
            Err(Error::new(ErrorKind::NotConnected, "Bad credentials"))
        } else if t.eq("mysql") {
            if let Ok(s) = run_mysql(&app) {
                return s.await;
            }
            Err(Error::new(ErrorKind::NotConnected, "Bad credentials"))
        } else if t.eq("sqlite") {
            if let Ok(s) = run_sqlite(&app) {
                return s.await;
            }
            Err(Error::new(ErrorKind::NotConnected, "Bad credentials"))
        } else {
            Err(Error::new(ErrorKind::NotFound, "Bad db type"))
        }
    } else {
        Err(Error::new(ErrorKind::NotConnected, "Bad db type"))
    }
}

fn run_sqlite(app: &ArgMatches) -> Result<Server, Error> {
    let database = app
        .get_one::<String>("database")
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Database name is missing"))?;
    let manager = ConnectionManager::<SqliteConnection>::new(format!("sqlite://{database}"));
    let pool = Pool::builder()
        .build(manager)
        .map_err(|_| Error::new(ErrorKind::NotConnected, "Bad credentials"))?;
    HttpServer::new(move || {
        App::new()
            .service(sqlite_welcome)
            .app_data(Data::new(pool.clone()))
    })
    .bind(("0.0.0.0", 8000))
    .map_or_else(
        |_| {
            Err(Error::new(
                ErrorKind::AddrNotAvailable,
                "Failed to run server",
            ))
        },
        |http| Ok(http.run()),
    )
}

fn run_postgres(app: &ArgMatches) -> Result<Server, Error> {
    let user = app
        .get_one::<String>("user")
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Username is missing"))?;
    let password = app
        .get_one::<String>("password")
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Password is missing"))?;
    let host = app
        .get_one::<String>("host")
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Host is missing"))?;
    let database = app
        .get_one::<String>("database")
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Database name is missing"))?;

    let manager = ConnectionManager::<PgConnection>::new(format!(
        "postgres://{user}:{password}@{host}/{database}"
    ));
    let pool = Pool::builder()
        .build(manager)
        .map_err(|_| Error::new(ErrorKind::NotConnected, "Bad credentials"))?;
    HttpServer::new(move || {
        App::new()
            .service(pgsql_welcome)
            .app_data(Data::new(pool.clone()))
    })
    .bind(("0.0.0.0", 8000))
    .map_or_else(
        |_| {
            Err(Error::new(
                ErrorKind::AddrNotAvailable,
                "Failed to run server",
            ))
        },
        |http| Ok(http.run()),
    )
}

fn run_mysql(app: &ArgMatches) -> Result<Server, Error> {
    let user = app
        .get_one::<String>("user")
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Username is missing"))?;
    let password = app
        .get_one::<String>("password")
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Password is missing"))?;
    let host = app
        .get_one::<String>("host")
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Host is missing"))?;
    let database = app
        .get_one::<String>("database")
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Database name is missing"))?;

    let manager = ConnectionManager::<MysqlConnection>::new(format!(
        "mysql://{user}:{password}@{host}/{database}"
    ));
    let pool = Pool::builder()
        .build(manager)
        .map_err(|_| Error::new(ErrorKind::NotConnected, "Bad credentials"))?;
    HttpServer::new(move || {
        App::new()
            .service(mysql_welcome)
            .app_data(Data::new(pool.clone()))
    })
    .bind(("0.0.0.0", 8000))
    .map_or_else(
        |_| {
            Err(Error::new(
                ErrorKind::AddrNotAvailable,
                "Failed to run server",
            ))
        },
        |http| Ok(http.run()),
    )
}
