use std::future::Future;

pub trait HttpRestClient {
    fn get(&self, path: &str) -> impl Future<Output = Result<Vec<u8>, String>>;
    fn post(&self, path: &str, body: &str) -> impl Future<Output = Result<Vec<u8>, String>>;
    fn patch(&self, path: &str, body: &str) -> impl Future<Output = Result<Vec<u8>, String>>;
    fn upsert(&self, path: &str, body: &str) -> impl Future<Output = Result<Vec<u8>, String>>;
    fn delete(&self, path: &str) -> impl Future<Output = Result<Vec<u8>, String>>;
}
