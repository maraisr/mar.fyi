extern crate http;

use std::collections::HashMap;

use http::{header, Request, Response, StatusCode};

fn handler(request: Request<()>) -> http::Result<Response<String>> {
    let mut map: HashMap<&'static str, &'static str> = HashMap::new();
    map.insert("twitter", "https://twitter.com/codervandal");
    map.insert("github", "https://github.com/maraisr");
    map.insert("paypal", "https://www.paypal.me/maraisr");
    map.insert("coffee", "https://www.buymeacoffee.com/marais");

    if cfg!(test) {
        map.insert("test_key", "test_url");
    }

    let uri = &request.uri().path()[1..];

    if let Some(point_to) = map.get(&uri) {
        println!("⚡️ Found: {}, sending: {}", uri, *point_to);

        return Response::builder()
            .status(StatusCode::FOUND)
            .header(header::LOCATION, *point_to)
            .header(
                header::CACHE_CONTROL,
                "public, s-maxage=43200, max-age=0, stale-while-revalidate",
            )
            .header(
                header::CONTENT_TYPE,
                "content-type: text/plain; charset=utf-8",
            )
            .body(format!("⚡️ Zapping you over to: {}", *point_to));
    } else {
        println!("🤔 Asking for: {}", uri);
    }

    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header(
            header::CACHE_CONTROL,
            "public, s-maxage=7200, max-age=0, stale-while-revalidate",
        )
        .header(
            header::CONTENT_TYPE,
            "content-type: text/plain; charset=utf-8",
        )
        .body(format!("❌ Nothing here to see..."))
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_should_work() {
        let resp = handler(
            Request::builder()
                .uri("/test_key")
                .body(())
                .unwrap()
        );

        assert_eq!(resp.unwrap().status(), StatusCode::FOUND);
    }

    #[test]
    fn it_should_have_location_header() {
        let resp = handler(
            Request::builder()
                .uri("/test_key")
                .body(())
                .unwrap()
        );

        assert_eq!(resp.unwrap().headers().get(header::LOCATION).is_some(), true);
    }

    #[test]
    fn it_should_fail() {
        let resp = handler(
            Request::builder()
                .uri("/abc123")
                .body(())
                .unwrap()
        );

        assert_eq!(resp.unwrap().status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn it_should_at_least_match_twitter() {
        let resp = handler(
            Request::builder()
                .uri("/twitter")
                .body(())
                .unwrap()
        );

        assert_eq!(resp.unwrap().status(), StatusCode::FOUND);
    }
}
