#![allow(clippy::multiple_crate_versions)]
use actix_web::web::Data;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use clap::{Arg, ArgMatches, Command};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{MysqlConnection, PgConnection, SqliteConnection};
use std::io::{Error, ErrorKind, Result};

#[get("/")]
async fn mysql_welcome(_p: Data<Pool<ConnectionManager<MysqlConnection>>>) -> impl Responder {
    HttpResponse::Ok().body("Welcome to mysql")
}

#[get("/")]
async fn postgres_welcome(_p: Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    HttpResponse::Ok().body("Welcome to postgres")
}

#[get("/")]
async fn sqlite_welcome(_p: Data<Pool<ConnectionManager<SqliteConnection>>>) -> impl Responder {
    HttpResponse::Ok().body("Welcome to sqlite")
}

async fn run_mysql(cmd: ArgMatches) -> Result<()> {
    if let Some(u) = cmd.get_one::<String>("user") {
        if let Some(p) = cmd.get_one::<String>("password") {
            if let Some(h) = cmd.get_one::<String>("host") {
                if let Some(d) = cmd.get_one::<String>("database") {
                    let manager = ConnectionManager::<MysqlConnection>::new(
                        format!("mysql://{u}:{p}@{h}/{d}").as_str(),
                    );
                    if let Ok(pool) = Pool::builder().build(manager) {
                        return HttpServer::new(move || {
                            App::new()
                                .app_data(Data::new(pool.clone()))
                                .service(mysql_welcome)
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
    Err(Error::new(ErrorKind::NotFound, "Username is missing"))
}

async fn run_postgres(cmd: ArgMatches) -> Result<()> {
    if let Some(u) = cmd.get_one::<String>("user") {
        if let Some(p) = cmd.get_one::<String>("password") {
            if let Some(h) = cmd.get_one::<String>("host") {
                if let Some(d) = cmd.get_one::<String>("database") {
                    let manager = ConnectionManager::<PgConnection>::new(
                        format!("postgres://{u}:{p}@{h}/{d}").as_str(),
                    );
                    if let Ok(pool) = Pool::builder().build(manager) {
                        return HttpServer::new(move || {
                            App::new()
                                .app_data(Data::new(pool.clone()))
                                .service(postgres_welcome)
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
    Err(Error::new(ErrorKind::NotFound, "Username is missing"))
}
async fn run_sqlite(cmd: ArgMatches) -> Result<()> {
    if let Some(d) = cmd.get_one::<String>("database") {
        let manager = ConnectionManager::<SqliteConnection>::new(format!("sqlite:{d}").as_str());
        if let Ok(pool) = Pool::builder().build(manager) {
            return HttpServer::new(move || {
                App::new()
                    .app_data(Data::new(pool.clone()))
                    .service(sqlite_welcome)
            })
            .bind(("127.0.0.1", 8000))?
            .run()
            .await;
        }
        return Err(Error::new(ErrorKind::NotConnected, "Bad credentials"));
    }
    Err(Error::new(ErrorKind::NotFound, "Database name is missing"))
}
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
#[actix_web::main]
async fn main() -> Result<()> {
    let cmd = aphrodite();
    if let Some(t) = cmd.get_one::<String>("type") {
        return if t.eq("mysql") {
            run_mysql(cmd).await
        } else if t.eq("postgres") {
            run_postgres(cmd).await
        } else if t.eq("sqlite") {
            run_sqlite(cmd).await
        } else {
            Err(Error::new(
                ErrorKind::NotConnected,
                "mysql postgres sqlite expected",
            ))
        };
    }
    Err(Error::new(ErrorKind::NotConnected, "Type is missing"))
}
