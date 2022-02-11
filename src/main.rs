//! # Xaytex-dashboard

mod appliction;

use actix_web::{App, HttpServer, web};
use dotenv::dotenv;



async fn main() -> stq_api::Result<()> {
    dotenv().ok();

    println!("Starting server...");
    println!("Starting application...")



}