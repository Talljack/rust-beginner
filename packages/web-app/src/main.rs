#[macro_use]
extern crate diesel;

mod error;
mod model;
mod schema;
mod control;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, get, post, middleware::Logger};
use log::info;
use error::MyError;
use diesel::{prelude::*, r2d2};
use control::{find_user_by_id, insert_new_user};
use dotenvy::dotenv;
use uuid::Uuid;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = init_pool();
    log::info!("Starting server at http://localhos:8000");
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(index))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn index() -> Result<HttpResponse, MyError> {
    // HttpResponse::Ok().body("Hey here!")
    let err = MyError::TimedOut;
    info!("{}", err);
    Err(err)
}

fn init_pool() -> DbPool {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(db_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

#[get("/user/{user_id}")]
async fn get_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let user_uid = user_id.into_inner();
    let user = web::block(move || {
        let mut conn = pool.get()?;
        find_user_by_id(&mut conn, user_uid)
    })
    .await?
    .map_err(MyError::InternalServerError)?;

    Ok(match user {
        Some(u) => HttpResponse::Ok().json(u),
        None => HttpResponse::NotFound().body("Not Found"),
    })
}

#[post("/user")]
async fn add_user(
    pool: web::Data<DbPool>,
    new_user: web::Json<model::NewUser>,
) -> actix_web::Result<impl Responder> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        insert_new_user(&mut conn, &new_user.name)
    })
    .await
    .map_err(MyError::InternalServerError)?;

    Ok(HttpResponse::Created().json(user))
}
