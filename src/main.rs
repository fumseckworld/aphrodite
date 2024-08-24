//!
//! # Aphrodite
//!
//! > API in order to manage database
//!
#![allow(clippy::multiple_crate_versions)]

use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{main, App, HttpServer};
use clap::{Arg, ArgMatches, Command};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{MysqlConnection, PgConnection, SqliteConnection};
use std::io::{Error, ErrorKind};

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
        .arg(Arg::new("user").short('u').long("user").required(true))
        .arg(
            Arg::new("password")
                .short('p')
                .long("password")
                .required(true),
        )
        .arg(Arg::new("host").short('h').long("host").required(true))
        .get_matches()
}
#[doc = "The api entry"]
#[main]
async fn main() -> Result<(), Error> {
    let app = aphrodite();
    if let Some(t) = app.get_one::<String>("type") {
        if t.eq(&"postgres") {
            if let Ok(s) = run_postgres(&app) {
                s.await
            } else {
                Err(Error::new(ErrorKind::AddrInUse, "Server not running"))
            }
        } else if t.eq(&"mysql") {
            if let Ok(s) = run_mysql(&app) {
                s.await
            } else {
                Err(Error::new(ErrorKind::AddrInUse, "Server not running"))
            }
        } else if t.eq(&"sqlite") {
            if let Ok(s) = run_sqlite(&app) {
                s.await
            } else {
                Err(Error::new(ErrorKind::AddrInUse, "Server not running"))
            }
        } else {
            Err(Error::new(ErrorKind::NotFound, "Database type is missing"))
        }
    } else {
        Err(Error::new(ErrorKind::NotFound, "Database type is missing"))
    }
}

fn run_sqlite(app: &ArgMatches) -> Result<Server, Error> {
    if let Some(d) = app.get_one::<String>("database") {
        let manager = ConnectionManager::<SqliteConnection>::new(format!("sqlite:{d}").as_str());
        if let Ok(pool) = Pool::builder().build(manager) {
            if let Ok(http) = HttpServer::new(move || App::new().app_data(Data::new(pool.clone())))
                .bind(("0.0.0.0:8000", 8000))
            {
                return Ok(http.run());
            }
        }
        return Err(Error::new(ErrorKind::NotConnected, "Bad credentials"));
    }
    Err(Error::new(ErrorKind::NotFound, "Database name is missing"))
}

fn run_postgres(app: &ArgMatches) -> Result<Server, Error> {
    if let Some(u) = app.get_one::<String>("user") {
        if let Some(p) = app.get_one::<String>("password") {
            if let Some(h) = app.get_one::<String>("host") {
                if let Some(d) = app.get_one::<String>("database") {
                    let manager = ConnectionManager::<PgConnection>::new(
                        format!("postgres://{u}:{p}@{h}/{d}").as_str(),
                    );
                    if let Ok(pool) = Pool::builder().build(manager) {
                        if let Ok(http) =
                            HttpServer::new(move || App::new().app_data(Data::new(pool.clone())))
                                .bind(("0.0.0.0", 8000))
                        {
                            return Ok(http.run());
                        }
                        return Err(Error::new(
                            ErrorKind::NotConnected,
                            "Failed to run the server",
                        ));
                    }
                    return Err(Error::new(ErrorKind::NotConnected, "Bad credentials"));
                }
                return Err(Error::new(ErrorKind::NotFound, "Database name is missing"));
            }
        }
        return Err(Error::new(ErrorKind::NotFound, "Password is missing"));
    }
    Err(Error::new(ErrorKind::NotFound, "Username is missing"))
}

fn run_mysql(app: &ArgMatches) -> Result<Server, Error> {
    if let Some(u) = app.get_one::<String>("user") {
        if let Some(p) = app.get_one::<String>("password") {
            if let Some(h) = app.get_one::<String>("host") {
                if let Some(d) = app.get_one::<String>("database") {
                    let manager = ConnectionManager::<MysqlConnection>::new(
                        format!("mysql://{u}:{p}@{h}/{d}").as_str(),
                    );
                    if let Ok(pool) = Pool::builder().build(manager) {
                        if let Ok(http) =
                            HttpServer::new(move || App::new().app_data(Data::new(pool.clone())))
                                .bind(("0.0.0.0:8000", 8000))
                        {
                            return Ok(http.run());
                        }
                        return Err(Error::new(
                            ErrorKind::NotConnected,
                            "Failed to run the server",
                        ));
                    }
                    return Err(Error::new(ErrorKind::NotConnected, "Bad credentials"));
                }
                return Err(Error::new(ErrorKind::NotFound, "Database name is missing"));
            }
        }
        return Err(Error::new(ErrorKind::NotFound, "Password is missing"));
    }
    Err(Error::new(ErrorKind::NotFound, "Username is missing"))
}
