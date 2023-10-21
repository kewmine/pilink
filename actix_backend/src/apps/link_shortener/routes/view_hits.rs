use actix_web::{get, HttpResponse, Responder, web};
use actix_web::web::{Data};
use mongodb::bson::{doc, Document};
#[allow(unused_imports)]
use actix_cors;
use mongodb::Collection;
use crate::apps::link_shortener::libs::config::DBConfigFields;


#[get("/hits")]
async fn view_hits_page() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../webpages/hits.html"))
}


#[get("/{hash}/hits")]
async fn view_hits(
    path: web::Path<String>,
    db_session: Data<Collection<Document>>,
    toml_data: Data<DBConfigFields>,
) -> impl Responder {
    let hash = &path.into_inner();
    let mongodb_collection = &db_session.into_inner();
    let db_config = &toml_data.into_inner();

    // get longuri for quarried hash from db
    let fetcher = mongodb_collection.find_one(
        doc! {
            &db_config.dbfield_shortlink_hash:&hash
        }, None,
    ).await
        .expect("Error processing query -> fetch shortlink hash");

    // if doc was found, redirect to longuri else redirect to 404
    // and increment "hits" if counting is enabled
    match fetcher {
        Some(doc) => {

            // check if hits counter is enabled
            let counting_enabled = doc.get(&db_config.dbfield_is_counting_hits).unwrap().as_bool().unwrap();

            // if enabled, return hits
            match counting_enabled {
                true => {
                    let hits = doc.get_i64(&db_config.dbfield_hits).unwrap();
                    HttpResponse::Ok().json(hits )
                },

                false => {
                    HttpResponse::Ok().json(false )
                },
            }
        }

        // if doc is not found, return msg
        None => HttpResponse::NotFound().json("link with that id does not exist")
    }
}