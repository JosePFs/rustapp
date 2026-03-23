use std::future::Future;

use crate::ports::error::Result;

pub trait HttpRestClient {
    fn get(&self, path: &str) -> impl Future<Output = Result<Vec<u8>>>;
    fn post(&self, path: &str, body: &str) -> impl Future<Output = Result<Vec<u8>>>;
    fn patch(&self, path: &str, body: &str) -> impl Future<Output = Result<Vec<u8>>>;
    fn upsert(&self, path: &str, body: &str) -> impl Future<Output = Result<Vec<u8>>>;
    fn delete(&self, path: &str) -> impl Future<Output = Result<Vec<u8>>>;
}
