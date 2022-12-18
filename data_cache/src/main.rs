use actix_web::{middleware::Logger, web, App, HttpServer};
use mongodb::Client;
pub mod handlers;
pub mod models;
use handlers::event_handler::{add_event, get_event};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let host = env::var("HOST").expect("Error Reading HOST Env Variable");
    let port = env::var("PORT").expect("Error Reading PORT Env Variable");
    let db_url = env::var("DATABASE_URL").expect("Error Reading DB Env Variable");
    let db_name = env::var("DATABASE_NAME").expect("Error Reading DB Name Env Variable");
    let db = Client::with_uri_str(&db_url)
        .await
        .expect("Error connecting to DB Client")
        .database(&db_name);
    let db = web::Data::new(db);
    let ip_port = format!("{}:{}", host, port);
    println!("server running on : {ip_port}");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(db.clone())
            .service(add_event)
            .service(get_event)
    })
    .bind(ip_port)
    .expect("Error binding to ip_port : ) ")
    .run()
    .await
}
