extern crate actix_web;

use std::collections::HashMap;

use actix_web::{actix, App, http, HttpResponse, Path, server};

static LISTEN_ON: &'static str = "0.0.0.0:8080";

fn main() {
    let sys = actix::System::new("app");

    server::new(
        || App::new()
            .route("/{slug}", http::Method::GET, handler))
        .bind(LISTEN_ON).unwrap()
        .start();

    println!("🚀 Started server on: {}", LISTEN_ON);

    sys.run();
}

fn handler(slug: Path<String>) -> HttpResponse {
    let mut map: HashMap<&'static str, &'static str> = HashMap::new();
    map.insert("twitter", "https://www.twitter.com");

    if cfg!(test) {
        map.insert("test", "test");
    }

    if let Some(point_to) = map.get(&slug[..]) {
        println!("⚡️ Found: {}, sending: {}", slug, *point_to);

        return HttpResponse::Found()
            .header(http::header::LOCATION, *point_to)
            .header(http::header::CACHE_CONTROL, "public, s-maxage=43200, maxage=43200")
            .finish();
    } else {
        println!("🤔 Asking for: {}", slug);
    }

    HttpResponse::NotFound()
        .finish()
}
