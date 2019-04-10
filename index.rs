extern crate actix_web;
extern crate env_logger;
extern crate log;

use std::collections::HashMap;

use actix_web::{actix, App, http, HttpResponse, Path, server};

static LISTEN_ON: &'static str = "0.0.0.0:8080";

fn main() {
    env_logger::init_from_env(
        env_logger::Env::default()
            .filter_or("LOG_LEVEL", "app=debug")
    );

    let sys = actix::System::new("app");

    server::new(|| {
        App::new()
            .route("/{slug}", http::Method::GET, handler)
    })
        .bind(LISTEN_ON)
        .unwrap()
        .start();

    log::info!("🚀 Started server on: {}", LISTEN_ON);

    sys.run();
}

fn handler(slug: Path<String>) -> HttpResponse {
    let mut map: HashMap<&'static str, &'static str> = HashMap::new();
    map.insert("twitter", "https://twitter.com/codervandal");
    map.insert("github", "https://github.com/maraisr");

    if let Some(point_to) = map.get(&slug[..]) {
        log::info!("⚡️ Found: {}, sending: {}", slug, *point_to);

        return HttpResponse::Found()
            .header(http::header::LOCATION, *point_to)
            .header(
                http::header::CACHE_CONTROL,
                "public, s-maxage=43200, max-age=43200, must-revalidate",
            )
            .header(
                http::header::CONTENT_TYPE,
                "content-type: text/plain; charset=utf-8",
            )
            .body(format!("⚡️ Zapping you over to: {}", *point_to));
    } else {
        log::info!("🤔 Asking for: {}", slug);
    }

    HttpResponse::NotFound().finish()
}
