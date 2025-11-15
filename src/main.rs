mod error;
mod models;
mod cache;

use error::{GranolaError, Result};
use models::*;
use cache::{load_cache, resolve_cache_path};

fn main() {
    println!("Hello, world!");
}
