extern crate hyper;
extern crate hyper_tls;
extern crate futures;

use std::sync::Arc;
use llspl::syntax::*;
use hyper::{Request,Body};
use hyper::rt::{Future, Stream};

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
    client: Arc<HttpsClient>
}

impl Invoker for GoogleCloudFunctions {
    fn invoke<'a,'b>(&'b self, name: &'b str, input: Payload) -> EvalResult<'a> {
        let mut url = String::new();
        url.push_str(&self.uri_base);
        url.push_str(name);
        let client = self.client.clone();
        let req = Request::builder()
            .method("POST")
            .uri(url)
            .body(input.to_body());
        Box::new(futures::done(req)
            .map_err(|err| { () })
            .and_then(move |req| { client.request(req).map_err(|err| { () }) })
            .and_then(|response| { 
                let (headers, body) = response.into_parts();
                body.concat2()
                    .map(|chunk| {
                        Payload::Chunk(chunk)
                    })
                    .map_err(|err| { () })
            }))
    }    
}

pub fn new(project: &str, zone: &str) -> GoogleCloudFunctions  {
    let uri_base = format!("https://{}-{}.cloudfunctions.net/", 
        zone, project);
    let https = hyper_tls::HttpsConnector::new(4).unwrap();
    let client = hyper::Client::builder()
            .build::<_, Body>(https);
    GoogleCloudFunctions { 
        uri_base: uri_base.into_boxed_str(), 
        client : Arc::new(client)
    }
}
