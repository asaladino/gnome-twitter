#![deny(warnings)]
extern crate hyper;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[cfg(test)]
mod test_twitter {
    use hyper::{Client, Method, Request, header};
    use hyper::rt::{self, Future, Stream};
    use hyper_tls::HttpsConnector;

    // https://github.com/hyperium/hyper/blob/master/examples/client_json.rs
    // https://developer.twitter.com/en/docs/basics/authentication/overview/3-legged-oauth

    #[test]
    fn twitter_login() {
        // Twitter 3-legged authorization
        let url = "https://api.twitter.com/oauth/request_token?oauth_consumer_key=I09YHsVzXWdbZG8vOchVz7f0d&".parse().unwrap();
        static POST_DATA: &str = r#""#;
        let req = Request::builder()
            .method(Method::POST)
            .uri(url)
            .header(header::CONTENT_TYPE, "application/json")
            .body(POST_DATA.into())
            .unwrap();

        let fut = post_json(req)
            .map(|response| {
                println!("{:#?}", response);
            })
            .map_err(|e| {
                match e {
                    FetchError::Http(e) => eprintln!("http error: {}", e),
                    FetchError::Json(e) => eprintln!("json parsing error: {}", e),
                }
            });

        rt::run(fut);
    }

    fn fetch_json(url: hyper::Uri) -> impl Future<Item=Vec<User>, Error=FetchError> {
        let https = HttpsConnector::new(4).expect("TLS initialization failed");

        let client = Client::builder().build::<_, hyper::Body>(https);

        client
            // Fetch the url...
            .get(url)
            // And then, if we get a response back...
            .and_then(|res| {
                // asynchronously concatenate chunks of the body
                res.into_body().concat2()
            })
            .from_err::<FetchError>()
            // use the body after concatenation
            .and_then(|body| {
                // try to parse as json with serde_json
                let users = serde_json::from_slice(&body)?;

                Ok(users)
            })
            .from_err()
    }

    fn post_json(req: Request<&str>) -> impl Future<Item=Vec<User>, Error=FetchError> {
        let https = HttpsConnector::new(4).expect("TLS initialization failed");

        let client = Client::builder().build::<_, hyper::Body>(https);

        Box::new(client.request(req).from_err().map(|web_res| {
            // Compare the JSON we sent (before) with what we received (after):
            let body = Body::wrap_stream(web_res.into_body().map(|b| {
                Chunk::from(format!("<b>POST request body</b>: {}<br><b>Response</b>: {}",
                                    POST_DATA,
                                    std::str::from_utf8(&b).unwrap()))
            }));
        }))
    }

    #[derive(Deserialize, Debug)]
    struct User {
        id: i32,
        name: String,
    }

    // Define a type so we can return multiple types of errors
    enum FetchError {
        Http(hyper::Error),
        Json(serde_json::Error),
    }

    impl From<hyper::Error> for FetchError {
        fn from(err: hyper::Error) -> FetchError {
            FetchError::Http(err)
        }
    }

    impl From<serde_json::Error> for FetchError {
        fn from(err: serde_json::Error) -> FetchError {
            FetchError::Json(err)
        }
    }
}
