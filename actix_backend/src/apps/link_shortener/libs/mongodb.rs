use mongodb::{Collection, bson::Document};
use crate::apps::link_shortener::libs::config::DBConfigFields;

pub async fn collection(config: &DBConfigFields) -> Collection<Document> {
    let links_db: Collection<Document> = mongodb::Client::with_uri_str(&config.db_uri).await
        .expect("error connecting to links database")
        .database(&config.db_name).collection(&config.db_coll_name);
    links_db
}
