extern crate hyper;
extern crate hyper_tls;
extern crate futures;
extern crate s3;

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

impl Eval for GoogleCloudFunctions {

    fn fetch<'a,'b>(&'b self, path: &'b str) -> EvalResult<'a> {
        Box::new(futures::future::result(self.storage.fetch(path)))
    }

    fn invoke<'a,'b>(&'b self, name: &'b str, input: Payload) -> EvalResult<'a> {
        let mut url = String::new();
        url.push_str(&self.uri_base);
        url.push_str(name);
        let client = self.client.clone();
        let req = input.to_body()
          .and_then(|chunk|
            Request::builder()
            .method("POST")
            .uri(url)
            .body(Body::from(chunk))
            .map_err(|err| Error::Http(err)));
        let resp = futures::future::result(req)
            .and_then(move |req| client.request(req).map_err(|err| Error::Hyper(err)))
            .and_then(|response| { 
                let (_headers, body) = response.into_parts();
                body.concat2().map_err(|err| Error::Hyper(err))
                    .map(|chunk| Payload::Chunk(chunk))
            });
        Box::new(resp)
    }
}

pub fn new(project: &str, zone: &str, storage: Storage) -> GoogleCloudFunctions  {
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
