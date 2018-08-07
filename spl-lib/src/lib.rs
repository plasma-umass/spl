#[macro_use]
extern crate nom;
extern crate serde_json;
extern crate hyper;
extern crate hyper_tls;
extern crate futures;
extern crate s3;
extern crate chrono;
use hyper::{Request,Body,Client};

use std::default::Default;


pub mod json_transformers;
pub mod llspl;

fn test_storage()  {
  use s3;

  let cred = s3::credentials::Credentials::new(
    Some(String::from("GOOGDW42FWZANYAZVLIS")), 
    Some(String::from("Zb8GQEtzGyr2aRGDKWOJARTegxqU6ntqNT6gce/A")),
    None, None);
  let bucket_name = "plasma-tmp";
  let region = s3::region::Region::Custom(String::from("storage.googleapis.com"));
  

  let bucket = s3::bucket::Bucket::new(bucket_name, region, cred);
  let (data, code) = bucket.get("calvin.png").unwrap();
  println!("Code: {}\n", data.len());
  assert!(false);
}
