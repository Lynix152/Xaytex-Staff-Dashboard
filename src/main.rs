//! # Xaytex-dashboard

use actix_web::{App, HttpServer, web};
use dotenv::dotenv;



async fn main() -> stq_api::Result<()> {
    dotenv().ok();

    HttpServer::new(|| App::new()
        .route("/task", web::get().to(handler::task::get_all))
        .route("/task/{id}", web::get().to(handler::task::get_by_id))
        .route("/task", web::post().to(handler::task::create_task))
    )
        .bind("127.0.0.1:8080")?
        .run()
        .await

}