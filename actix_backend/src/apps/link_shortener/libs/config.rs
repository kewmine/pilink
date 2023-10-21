use {std::fs, serde::Deserialize, toml};

// toml key: [links]
#[derive(Deserialize)]
pub struct Links {
    links: DBConfigFields,
}

// toml variables
#[derive(Deserialize, Clone)]
pub struct DBConfigFields {
    pub domain: String,
    pub db_uri: String,
    pub db_name: String,
    pub db_coll_name: String,
    pub dbfield_shortlink_domain: String,
    pub dbfield_shortlink_hash: String,
    pub dbfield_longlink_uri: String,
    pub dbfield_created_on: String,
    pub dbfield_is_counting_hits: String,
    pub dbfield_hits: String,
    pub dbfield_last_hit_on: String,
}

pub fn toml_data(toml_file: &str) -> DBConfigFields {
    // read file contents to string
    let content = match fs::read_to_string(toml_file) {
        Ok(c) => c,
        Err(_) => panic!("Parsing Error: could not parse {}", toml_file)
    };

    let data: Links = match toml::from_str(&*content) {
        Ok(d) => d,
        Err(_) => panic!("Parsing Error: could not parse data from {}", toml_file),
    };

    data.links
}
