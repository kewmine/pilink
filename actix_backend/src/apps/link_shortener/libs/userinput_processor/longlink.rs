use std::str::FromStr;
use actix_web::http::Uri;
use actix_web::web::Data;
use mongodb::bson::{doc, Document};
use mongodb::Collection;
use crate::apps::link_shortener::libs::config::DBConfigFields;

pub fn sanitize(uri: &mut String) {
    // prepend http:// if no protocol found
    match uri.starts_with("http://") || uri.starts_with("https://") {
        true => {}
        false => *uri = format!("http://{}", uri),
    };
}

// returns bool based on uri format validity
pub fn is_valid(uri: &String) -> bool {
    let format_valid = Uri::from_str(&uri);
    match format_valid {
        Ok(_) => true,
        Err(_) => false,
    }
}

// if a duplicate returns shortlink associated or None
pub async fn is_duplicate(
    uri: &String,
    toml_data: &Data<DBConfigFields>,
    coll: &Data<Collection<Document>>,
) -> Option<String> {
    let filter = doc! { &toml_data.dbfield_longlink_uri: uri };
    let fetcher = &coll
        .find_one(filter, None).await
        .expect("Validation Exception: error trying to check if longlink uri exists");

    // check if uri is a duplicate in db
    match fetcher {
        Some(doc) => {
            let shortlink_uri = format!(
                "uri submitted already exists with us,\n\t--> {}/{}\n",
                &doc.get_str(&toml_data.dbfield_shortlink_domain).unwrap(),
                &doc.get_str(&toml_data.dbfield_shortlink_hash).unwrap()
            );
            Some(shortlink_uri)
        }
        None => None
    }
}

// validate and sanitize input uri sequentially
pub async fn process(
    mut form_uri: &mut String,
    toml_data: &Data<DBConfigFields>,
    coll: &Data<Collection<Document>>,
) -> Option<String> {

    // sanitize form input
    sanitize(&mut form_uri);

    // if uri format is invalid, return Some type string or do nothing
    let uri_valid = is_valid(&form_uri);
    match uri_valid {
        true => {}
        false => return Some("uri submitted is invalid x_x".to_string()),
    }

    // if uri is a duplicate in db, return the shortlink associated or do nothing
    let uri_duplicate = is_duplicate(&form_uri, &toml_data, &coll).await;
    match uri_duplicate {
        Some(msg) => return Some(msg),
        None => {}
    }

    // return none if no exceptions returned
    None
}