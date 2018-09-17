extern crate hyper;
extern crate hyper_tls;
extern crate futures;
extern crate s3;
extern crate http;

use std::sync::Arc;
use super::syntax::*;
use super::storage::Storage;
use hyper::{Request,Body};
use hyper::rt::{Future, Stream};
use super::eval::{Eval, EvalResult};
use super::error::Error;

// No alias for this type? It seems quite fundamental.
type HttpsClient = hyper::Client<hyper_tls::HttpsConnector<
    hyper::client::HttpConnector>>;


pub struct GoogleCloudFunctions {
    uri_base: Box<str>,
    // The thread that calls GoogleCloudFunctions::new may die before the
    // worker threads that make requests. Therefore, we have to either
    // reference count the entire GoogleCloudFunctions structure, or just
    // reference count .client. If we end up using Arc<...> on more fields
    // in the future, it may make sense to reference count the entire structure.
    client: Arc<HttpsClient>,
    storage: Storage
}

// Hyper does not set Content-Length: 0 if the body is empty. GCF requires it.
fn set_content_length<'a, 'b>(
    body: &'b Body,
    builder: &'a mut http::request::Builder) -> &'a mut http::request::Builder {
    use hyper::body::Payload;
    match body.content_length() {
        None => builder,
        Some(n) => builder.header("Content-Length", n.to_string().as_str())
    }
}

fn get_content_type<'a, 'b>(payload : &'a Payload) -> &'b str {
    match payload {
        Payload::Chunk(_) => "text/plain",
        Payload::Json(_) => "application/json"
    }
}

impl Eval for GoogleCloudFunctions {

    fn fetch<'a,'b>(&'b self, path: &'b str) -> EvalResult<'a> {
        Box::new(futures::future::result(self.storage.fetch(path)))
    }

    fn invoke<'a,'b>(&'b self, name: &'b str, input: Payload) -> EvalResult<'a> {
        let url = format!("{}{}/", &self.uri_base, name);
        let client = self.client.clone();
        let content_type = get_content_type(&input);
        let req = input.to_body()
            .and_then(|chunk| {
                let mut builder = Request::builder();
                let builder = builder
                    .method("POST")
                    .uri(&url);
                let builder = set_content_length(&chunk, builder);
                builder
                .header("Content-Type", content_type)
                .body(Body::from(chunk))
                .map_err(|err| Error::Http(err))
            });
        let resp = futures::future::result(req)
            .and_then(move |req| client.request(req).map_err(|err| Error::Hyper(err)))
            .map(move |response| {
                  if !response.status().is_success() {
                    let msg = format!("{} from  {}", response.status(), url);
                    return Result::Err(Error::InvokeError(msg));
                  }
                  Result::Ok(response)
            })
            .flatten()
            .and_then(|response| {
                let (_headers, body) = response.into_parts();
                body.concat2().map_err(|err| Error::Hyper(err))
                  .map(|chunk| Payload::Chunk(chunk))
            });
        Box::new(resp)
    }
}

impl GoogleCloudFunctions {
    pub fn new(project: &str, zone: &str, storage: Storage) -> Self  {
        let uri_base = format!("https://{}-{}.cloudfunctions.net/",
            zone, project);
        let https = hyper_tls::HttpsConnector::new(4).unwrap();
        let client = hyper::Client::builder()
                .build::<_, Body>(https);
        GoogleCloudFunctions {
            uri_base: uri_base.into_boxed_str(),
            client : Arc::new(client),
            storage: storage
        }
    }
}
