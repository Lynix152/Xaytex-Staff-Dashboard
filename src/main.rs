//! # Xaytex-dashboard

#[warn(dead_code)]

mod contributor;
mod data;
//mod widgets;
//mod infrastructure;

fn main() {
    contributor::contributor();
    //infrastructure::server;
    data::db();
}
