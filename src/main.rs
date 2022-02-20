//! # Xaytex-dashboard


mod vpn;
mod application;
mod tcp;
mod error;
//mod error;


use dotenv::dotenv;



fn main() {
    dotenv().ok();

    println!("Starting server...");
    println!("Starting application...");
    get_version();
    get_contributors()
}

pub fn get_version() {
    let version_cargo = env!("CARGO_PKG_VERSION");
    println!("Version: {}", version_cargo);
}

pub fn get_contributors() {
    let contributors_cargo = env!("CARGO_PKG_AUTHORS");
    println!("Contributors: {}", contributors_cargo);
}