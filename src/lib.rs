use std::{env, ffi::OsStr};

pub mod jwt;
pub mod server;

pub use micros;

#[macro_use]
extern crate lazy_static;

fn ensure_var<K: AsRef<OsStr>>(key: K) -> anyhow::Result<String> {
    env::var(&key).map_err(|e| anyhow::anyhow!("{}: {:?}", e, key.as_ref()))
}

lazy_static! {
    static ref AUTH_SERVICE_URL: String = ensure_var("AUTH_SERVICE_URL").unwrap();
    static ref CSV_SERVICE_URL: String = ensure_var("CSV_SERVICE_URL").unwrap();    
}
