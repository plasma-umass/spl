#[macro_use]
extern crate nom;
#[macro_use]
extern crate serde_json;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate leak;
extern crate rpds;
extern crate s3;
extern crate reqwest;

mod json_transformers;
mod llspl;

use futures::future;
use hyper::rt::{Future, Stream};
use hyper::server::Server;
use hyper::service::service_fn;
use hyper::{Body, Request, Response, StatusCode};
use leak::Leak;
use rpds::HashTrieMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

// TODO(arjun): POC for storage, but not setup to work right now.
fn _test_storage() {
    use s3;

    let cred = s3::credentials::Credentials::new(
        Some(String::from("GOOGDW42FWZANYAZVLIS")),
        Some(String::from("Zb8GQEtzGyr2aRGDKWOJARTegxqU6ntqNT6gce/A")),
        None,
        None,
    );
    let bucket_name = "plasma-tmp";
    let region = s3::region::Region::Custom(String::from("storage.googleapis.com"));

    let bucket = s3::bucket::Bucket::new(bucket_name, region, cred);
    let (data, _code) = bucket.get("calvin.png").unwrap();
    println!("Code: {}\n", data.len());
    assert!(false);
}

// Executes an LLSPL program on input provided by a web request. The function
// takes a dictionary programs and an evaluator for an arbitrary platform.
fn handler<'a, E>(
    evaluator: &'static E,
    programs: &'static HashTrieMap<String, llspl::Expr>,
    req: Request<Body>,
) -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send + 'a>
where
    E: llspl::Eval,
{
    let (parts, body) = req.into_parts();
    let name = &parts.uri.path()[1..];
    match programs.get(name) {
        Option::None => {
            let resp = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from(format!("{} is not a known LLSPL program", name)))
                .unwrap();
            Box::new(future::ok(resp))
        }
        Option::Some(expr) => {
            let resp = body.concat2().and_then(move |body| {
                evaluator
                    .eval(llspl::Payload::Chunk(body), expr)
                    .map(|result| result.to_response())
                    .flatten()
                    .or_else(|err| {
                        let msg = String::from(err.description());
                        future::ok(
                            Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(msg))
                                .unwrap(),
                        )
                    })
            });
            Box::new(resp)
        }
    }
}

// Expects a list of programs as arguments. The filename is treated as the
// name of the function.
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut is_debug = false;
    let programs: HashTrieMap<_, _> = args.into_iter()
        .skip(1)
        .flat_map(|path| {
            if path == "-d" {
                is_debug = true;
                return None
            }

            let mut file = File::open(&path)
                .expect(&format!("Could not open {}", path));
            let mut buf = String::new();
            file.read_to_string(&mut buf)
                .expect(&format!("Could not read {}", path));
            let expr = llspl::parse(&buf)
                .expect(&format!("Error parsing {}", path))
                .1;
            Some((path, expr))
        })
        .collect();

    let programs = Box::new(programs);
    let programs = programs.leak();

    let evaluator = llspl::GoogleCloudFunctions::new(
        "umass-plasma",
        "us-east1",
        llspl::Storage::new(
            String::from("GOOGDW42FWZANYAZVLIS"),
            String::from("Zb8GQEtzGyr2aRGDKWOJARTegxqU6ntqNT6gce/A"),
            String::from("storage.googleapis.com"),
            "plasma-tmp",
        ),
        is_debug
    );
    let evaluator = Box::new(evaluator).leak();

    let addr = ([127, 0, 0, 1], 8000).into();
    let service = move || {
        service_fn(move |req| handler(evaluator, programs, req))
    };

    let server = Server::bind(&addr).serve(service).map_err(|_e| ());

    if !is_debug {
        println!("Listening on http://{}", addr);
    }

    hyper::rt::run(server)
}
