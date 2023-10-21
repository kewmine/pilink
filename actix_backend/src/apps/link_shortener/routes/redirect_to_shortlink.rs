use actix_web::{get, Responder, web};
use actix_web::web::{Data, Redirect};
use mongodb::bson::{doc, Document, DateTime, Bson};
use mongodb::Collection;
use crate::apps::link_shortener::libs::config::DBConfigFields;

// route to redirect to long uri
#[get("/{hash}")]
async fn redirect_to_shortlink(
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

            // increment "hits" if counting is enabled
            let counting_enabled = doc.get(&db_config.dbfield_is_counting_hits).unwrap();
            match counting_enabled {
                Bson::Boolean(true) => {
                    mongodb_collection.update_one(
                        doc! {&db_config.dbfield_shortlink_hash:&hash},
                        doc! {
                            "$inc":{&db_config.dbfield_hits:1}}, None).await.unwrap();
                }
                _ => {}
            };

            // update last hit
            mongodb_collection.update_one(
                doc! {&db_config.dbfield_shortlink_hash:&hash},
                doc! {
                    "$set":{&db_config.dbfield_last_hit_on:DateTime::now()}}, None,
            ).await.unwrap();

            // redirect to uri
            let uri = doc.get_str(&db_config.dbfield_longlink_uri).unwrap().to_string();
            Redirect::to(uri)
        }

        None => { Redirect::to("http://localhost:8080/404") }
    }
}