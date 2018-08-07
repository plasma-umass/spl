extern crate s3;

use s3::credentials::Credentials;
use s3::region::Region;
use s3::bucket::Bucket;
use super::syntax::{Payload, Error};

pub struct Storage {
    bucket: Bucket
}

impl Storage {

    pub fn new(access_key: String, 
           auth_key: String, 
           endpoint: String,
           bucket_name: &str) -> Storage {
        let cred = Credentials::new(Some(access_key), Some(auth_key), None,
                                    None);
        let region = Region::Custom(endpoint);
        let bucket = Bucket::new(bucket_name, region, cred);
        Storage { bucket }
    }

    pub fn fetch(&self, path: &str) -> Result<Payload, Error> {
        self.bucket.get(path)
            .map(|tuple| Payload::from_vec(tuple.0))
            .map_err(|err| Error::Storage(err))
    }

}
