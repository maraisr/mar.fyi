extern crate http;

use std::collections::HashMap;

use http::{Request, Response, StatusCode};

fn handler(request: Request<()>) -> http::Result<Response<()>> {
    let mut map = HashMap::new();
    map.insert("twitter", "https://www.twitter.com");

    if cfg!(test) {
        map.insert("test", "test");
    }

    let uri = &request.uri().path()[1..]; // the uri without the pre leading slash

    if let Some(point_to) = map.get(uri) {

        return Response::builder()
            .status(StatusCode::FOUND)
            .header("location", *point_to)
            .body(())
    }

    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_find_something() {
        let r = Request::builder()
            .uri("/test")
            .body(())
            .unwrap();

        assert_eq!(handler(r).unwrap().status(), StatusCode::FOUND);
    }

    #[test]
    fn it_should_have_location_header() {
        let r = Request::builder()
            .uri("/test")
            .body(())
            .unwrap();

        let handler_result = handler(r).unwrap();

        assert_eq!(handler_result.status(), StatusCode::FOUND);
        assert_eq!(handler_result.headers().get("location").is_some(), true);
    }

    #[test]
    fn it_should_404() {
        let r = Request::builder()
            .uri("/test2")
            .body(())
            .unwrap();

        assert_eq!(handler(r).unwrap().status(), StatusCode::NOT_FOUND);
    }
}
