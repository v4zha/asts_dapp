use crate::models::event_mdl::{Event, EventDB};
use actix_web::{get, post, web, Either, HttpResponse, Responder};
use mongodb::{bson::doc, options::FindOneOptions, Database};
#[post("/add_event")]
pub async fn add_event(p: web::Json<Event>, db: web::Data<Database>) -> impl Responder {
    let col = db.collection::<EventDB>("event_col");
    let post_db = EventDB::from(p.into_inner());
    match col.insert_one(&post_db, None).await {
        Ok(_) => HttpResponse::Ok().body("Event added successfully"),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}
#[get("/get_event")]
pub async fn get_event(db: web::Data<Database>) -> Either<web::Json<Event>, HttpResponse> {
    let col = db.collection::<EventDB>("event_col");
    let find_opt = FindOneOptions::builder()
        .sort(doc! { "$natural": -1 })
        .build();
    match col.find_one(None, find_opt).await {
        Ok(Some(res)) => Either::Left(web::Json(res.into())),
        Ok(None) => Either::Right(HttpResponse::Ok().body("No Info Found : )")),
        Err(err) => Either::Right(HttpResponse::NotFound().body(err.to_string())),
    }
}
