use serde::Deserialize;
use mongodb::{Collection, bson::{doc, Document, DateTime}};
use actix_web::{
    HttpResponse, post, Responder, web,
    web::Data,
};
use crate::{
    libs::hash_gen,
    link_shortener::libs::{userinput_processor, config::DBConfigFields},
};

#[derive(Deserialize)]
struct CreateShortlinkForm {
    uri: String,
    count_hits: bool,
}

// route to create new shortened uri
#[post("/new")]
async fn create_shortlink(
    form_data: web::Form<CreateShortlinkForm>,
    toml_data: Data<DBConfigFields>,
    coll: Data<Collection<Document>>,
) -> impl Responder {
    println!("{:?}", form_data.uri.as_str());

    // process uri
    let mut form_uri = form_data.uri.to_string();
    let form_uri_processor = userinput_processor::longlink::process(
        &mut form_uri,
        &toml_data,
        &coll,
    ).await;

    // if processor returns some exception,
    //  return http response or do nothing
    match form_uri_processor {
        Some(exception) => return HttpResponse::Ok().body(exception),
        None => {}
    }

    // process hash
    // processor mutates hash if its a duplicate in db
    let mut hash = hash_gen::alphanumeric(6);
    userinput_processor::shortlink::process(
        &mut hash,
        &toml_data,
        &coll,
    ).await;

    // craft the document with processed data
    let zero: i64 = 0;
    let doc = doc! {
        &toml_data.dbfield_created_on: DateTime::now(),
        &toml_data.dbfield_longlink_uri: &form_uri,
        &toml_data.dbfield_shortlink_domain: &toml_data.domain,
        &toml_data.dbfield_shortlink_hash: &hash,
        &toml_data.dbfield_is_counting_hits: &form_data.count_hits,
        &toml_data.dbfield_hits: zero,
        &toml_data.dbfield_last_hit_on:DateTime::now(),
    };

    // insert into database
    let insertion_status = &coll.insert_one(&doc, None).await;
    match insertion_status {
        Ok(_) => {
            let msg = format!("here is your shortened link\n\t--> {}/{}\n",
                              &doc.get_str("shortlink_domain").unwrap(),
                              &doc.get_str("shortlink_hash").unwrap());
            HttpResponse::Ok().body(msg)
        }
        Err(error) => {
            println!("Error while trying to insert a new link.\n{}", error.to_string());
            HttpResponse::Ok().body("Something went wrong x_x")
        }
    }
}
