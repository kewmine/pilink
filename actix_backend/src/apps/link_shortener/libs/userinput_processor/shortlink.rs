use actix_web::web::Data;
use mongodb::bson::{doc, Document};
use mongodb::Collection;
use crate::apps::link_shortener::libs::config::DBConfigFields;
use crate::libs::hash_gen;

pub async fn process(
    hash: &mut String,
    toml_data: &Data<DBConfigFields>,
    coll: &Data<Collection<Document>>,
) -> Option<String> {

    // if hash is a duplicate, mutate to a new hash or do nothing
    let shortlink_valid = is_duplicate(&hash, &toml_data, &coll).await;
    while shortlink_valid == false {
        *hash = hash_gen::alphanumeric(6);
    }
    None
}

async fn is_duplicate(
    hash: &String,
    toml_data: &Data<DBConfigFields>,
    coll: &Data<Collection<Document>>,
) -> bool {
    let filter = doc! {&toml_data.dbfield_shortlink_hash:&hash};
    let fetcher = coll
        .find_one(filter, None).await
        .expect("Exception while trying to fetch shortlink hash");

    match fetcher {
        Some(_) => false,
        None => true,
    }
}