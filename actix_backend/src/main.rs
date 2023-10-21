mod libs;
mod apps;
#[allow(unused_imports)]
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_settings::{ApplySettings, Settings};
use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use apps::link_shortener;

// this function could be located in a different module
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let config_file = "Architecture.toml";
    let actix_settings = Settings::parse_toml(config_file)
        .unwrap();

    // link shortener data
    let links_toml_data = link_shortener::libs::config::toml_data(config_file);
    let links_coll = link_shortener::libs::mongodb::collection(&links_toml_data).await;


    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())

            // origin = *
            //.wrap(Cors::permissive())

            .service(link_shortener::routes::new::create_shortlink)
            .service(link_shortener::routes::view_hits::view_hits)
            .service(link_shortener::routes::view_hits::view_hits_page)
            .service(link_shortener::routes::root::hello)
            .service(link_shortener::routes::root::not_found)
            .service(Files::new("/_app", "./src/apps/link_shortener/webpages/_app")
                .prefer_utf8(true)
                .show_files_listing())

            // static files

            // redirect_to_shortlink() matches all path
            .service(link_shortener::routes::redirect_to_shortlink::redirect_to_shortlink)
            // mongodb
            .app_data(Data::new(links_toml_data.to_owned()))
            .app_data(Data::new(links_coll.to_owned())
            )
    })
        .apply_settings(&actix_settings)

        .run()
        .await
}
