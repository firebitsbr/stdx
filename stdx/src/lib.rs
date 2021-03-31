//! The missing pieces of Rust's standard library

pub mod crypto;
pub mod encoding;
pub mod html;
pub mod job_scheduler;
pub mod mail;
pub mod markdown;
pub mod rand;
pub mod uuid;
pub mod validator;

pub use base64;
pub use chrono;
pub use csv;
pub use dotenv;
pub use env_logger;
pub use futures;
pub use image;
pub use lazy_static;
pub use log;
pub use num_cpus;
pub use regex;
pub use reqwest;
pub use sqlx;
pub use stdx_base32 as base32;
pub use stdx_otp as otp;
pub use stdx_qrcode as qrcode;
pub use stdx_stripe as stripe;
pub use stdx_taxes as taxes;
pub use stdx_threadpool as threadpool;
pub use stdx_ulid as ulid;
pub use tokio;
pub use tryhard as retry;
pub use url;
