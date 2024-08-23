#![allow(clippy::multiple_crate_versions)]
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use clap::{Arg, Command};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{MysqlConnection, PgConnection, SqliteConnection};
use std::io::{Error, ErrorKind};

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let cmd = Command::new("aphrodite")
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
        .get_matches();

    if let Some(t) = cmd.get_one::<String>("type") {
        if t.eq("mysql") {
            if let Some(u) = cmd.get_one::<String>("user") {
                if let Some(p) = cmd.get_one::<String>("password") {
                    if let Some(h) = cmd.get_one::<String>("host") {
                        if let Some(d) = cmd.get_one::<String>("database") {
                            let manager = ConnectionManager::<MysqlConnection>::new(
                                format!("{t}://{u}:{p}@{h}/{d}").as_str(),
                            );
                            if let Ok(pool) = Pool::builder().build(manager) {
                                return HttpServer::new(move || {
                                    App::new().app_data(Data::new(pool.clone()))
                                })
                                .bind(("127.0.0.1", 8000))?
                                .run()
                                .await;
                            }
                            return Err(Error::new(ErrorKind::NotConnected, "Bad credentials"));
                        }
                        return Err(Error::new(ErrorKind::NotFound, "Database name is missing"));
                    }
                }
                return Err(Error::new(ErrorKind::NotFound, "Password is missing"));
            }
            return Err(Error::new(ErrorKind::NotFound, "Username is missing"));
        } else if t.eq("postgres") {
            if let Some(u) = cmd.get_one::<String>("user") {
                if let Some(p) = cmd.get_one::<String>("password") {
                    if let Some(h) = cmd.get_one::<String>("host") {
                        if let Some(d) = cmd.get_one::<String>("database") {
                            let manager = ConnectionManager::<PgConnection>::new(
                                format!("{t}://{u}:{p}@{h}/{d}").as_str(),
                            );
                            if let Ok(pool) = Pool::builder().build(manager) {
                                return HttpServer::new(move || {
                                    App::new().app_data(Data::new(pool.clone()))
                                })
                                .bind(("127.0.0.1", 8000))?
                                .run()
                                .await;
                            }
                            return Err(Error::new(ErrorKind::NotConnected, "Bad credentials"));
                        }
                        return Err(Error::new(ErrorKind::NotFound, "Database name is missing"));
                    }
                }
                return Err(Error::new(ErrorKind::NotFound, "Password is missing"));
            }
            return Err(Error::new(ErrorKind::NotFound, "Username is missing"));
        } else if t.eq("sqlite") {
            if let Some(d) = cmd.get_one::<String>("database") {
                let manager =
                    ConnectionManager::<SqliteConnection>::new(format!("{t}:{d}").as_str());
                if let Ok(pool) = Pool::builder().build(manager) {
                    return HttpServer::new(move || App::new().app_data(Data::new(pool.clone())))
                        .bind(("127.0.0.1", 8000))?
                        .run()
                        .await;
                }
                return Err(Error::new(ErrorKind::NotConnected, "Bad credentials"));
            }
            return Err(Error::new(ErrorKind::NotFound, "Database name is missing"));
        }
    }
    Err(Error::last_os_error())
}
